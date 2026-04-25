use crate::lexer::{Lexer, Token, TokenType};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Double,
    String,
    Bool,
    Void,
    Class(String), // class name
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub return_type: Type,
    pub params: Vec<(String, Type)>,
    pub start_label: String,
    pub is_method: bool,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub members: Vec<(String, Type)>,
    pub methods: Vec<Function>,
}

pub struct Parser {
    lexer: Lexer,
    current: Token,
    variables: HashMap<String, (Type, i32)>, // type and offset
    functions: HashMap<String, Function>,
    classes: HashMap<String, Class>,
    stack_size: i32,
    label_count: i32,
    labels: HashMap<String, usize>,
    unresolved: HashMap<String, Vec<usize>>,
    code: Vec<u8>,
}

impl Parser {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer::new(input);
        let current = lexer.next_token();
        Parser {
            lexer,
            current,
            variables: HashMap::new(),
            functions: HashMap::new(),
            classes: HashMap::new(),
            stack_size: 0,
            label_count: 0,
            labels: HashMap::new(),
            unresolved: HashMap::new(),
            code: Vec::new(),
        }
    }

    fn eat(&mut self, token_type: &TokenType) {
        if self.current.token_type == *token_type {
            self.current = self.lexer.next_token();
        } else {
            panic!(
                "Parse error: expected {:?} but got {:?} ('{}')",
                token_type, self.current.token_type, self.current.lexeme
            );
        }
    }

    fn emit(&mut self, b: u8) {
        self.code.push(b);
    }

    fn emit32(&mut self, v: i32) {
        self.emit((v & 0xFF) as u8);
        self.emit(((v >> 8) & 0xFF) as u8);
        self.emit(((v >> 16) & 0xFF) as u8);
        self.emit(((v >> 24) & 0xFF) as u8);
    }

    fn emit64(&mut self, v: i64) {
        self.emit32((v & 0xFFFFFFFF) as i32);
        self.emit32((v >> 32) as i32);
    }

    fn new_label(&mut self) -> String {
        let label = format!("L{}", self.label_count);
        self.label_count += 1;
        label
    }

    fn emit_label(&mut self, label: &str) {
        self.labels.insert(label.to_string(), self.code.len());
    }

    fn emit_jne(&mut self, label: &str) {
        self.emit(0x0f);
        self.emit(0x85);
        self.unresolved
            .entry(label.to_string())
            .or_insert(Vec::new())
            .push(self.code.len());
        self.emit32(0);
    }

    fn emit_je(&mut self, label: &str) {
        self.emit(0x0f);
        self.emit(0x84);
        self.unresolved
            .entry(label.to_string())
            .or_insert(Vec::new())
            .push(self.code.len());
        self.emit32(0);
    }

    fn emit_jmp(&mut self, label: &str) {
        self.emit(0xe9);
        self.unresolved
            .entry(label.to_string())
            .or_insert(Vec::new())
            .push(self.code.len());
        self.emit32(0);
    }

    fn emit_call(&mut self, label: &str) {
        self.emit(0xe8);
        self.unresolved
            .entry(label.to_string())
            .or_insert(Vec::new())
            .push(self.code.len());
        self.emit32(0);
    }

    fn emit_ret(&mut self) {
        self.emit(0xc3);
    }

    fn emit_test_rax(&mut self) {
        self.emit(0x48);
        self.emit(0x85);
        self.emit(0xc0);
    }

    fn emit_mov_rax_imm(&mut self, val: i64) {
        self.emit(0x48);
        self.emit(0xb8);
        self.emit64(val);
    }

    fn emit_push_rax(&mut self) {
        self.emit(0x50);
    }

    fn emit_pop_rbx(&mut self) {
        self.emit(0x5b);
    }

    fn emit_pop_rax(&mut self) {
        self.emit(0x58);
    }

    fn emit_add_rax_rbx(&mut self) {
        self.emit(0x48);
        self.emit(0x01);
        self.emit(0xd8);
    }

    fn emit_sub_rax_rbx(&mut self) {
        self.emit(0x48);
        self.emit(0x29);
        self.emit(0xd8);
    }

    fn emit_mul_rbx(&mut self) {
        self.emit(0x48);
        self.emit(0xf7);
        self.emit(0xeb);
    }

    fn emit_div_rbx(&mut self) {
        self.emit(0x48);
        self.emit(0x99);
        self.emit(0x48);
        self.emit(0xf7);
        self.emit(0xfb);
    }

    fn emit_mov_rax_rbp_offset(&mut self, offset: i32) {
        self.emit(0x48);
        self.emit(0x8b);
        self.emit(0x85);
        self.emit32(-offset);
    }

    fn emit_mov_rbp_offset_rax(&mut self, offset: i32) {
        self.emit(0x48);
        self.emit(0x89);
        self.emit(0x85);
        self.emit32(-offset);
    }

    fn resolve_labels(&mut self) {
        for (label, positions) in &self.unresolved {
            if let Some(target) = self.labels.get(label) {
                for &pos in positions {
                    let offset = *target as i32 - (pos as i32 + 4);
                    self.code[pos] = (offset & 0xFF) as u8;
                    self.code[pos + 1] = ((offset >> 8) & 0xFF) as u8;
                    self.code[pos + 2] = ((offset >> 16) & 0xFF) as u8;
                    self.code[pos + 3] = ((offset >> 24) & 0xFF) as u8;
                }
            } else {
                panic!("Undefined label: {}", label);
            }
        }
    }

    pub fn generate_code(&mut self) -> Vec<u8> {
        self.code.clear();
        self.variables.clear();
        self.stack_size = 0;
        self.labels.clear();
        self.unresolved.clear();

        // Prologue
        self.emit(0x55); // push rbp
        self.emit(0x48);
        self.emit(0x89);
        self.emit(0xe5); // mov rbp, rsp

        self.parse_program();

        if self.stack_size > 0 {
            self.emit(0x48);
            self.emit(0x81);
            self.emit(0xec);
            self.emit32(self.stack_size);
        }

        // Epilogue
        self.emit(0x48);
        self.emit(0x89);
        self.emit(0xec); // mov rsp, rbp
        self.emit(0x5d); // pop rbp
        self.emit(0xc3); // ret

        self.resolve_labels();
        self.code.clone()
    }

    fn parse_program(&mut self) {
        while self.current.token_type != TokenType::End {
            match self.current.token_type {
                TokenType::Import => self.parse_import(),
                TokenType::Class => self.parse_class(),
                TokenType::Int
                | TokenType::Double
                | TokenType::String
                | TokenType::Bool
                | TokenType::Void => {
                    let next_token = {
                        let mut temp = self.lexer.clone();
                        temp.next_token()
                    };
                    if next_token.lexeme == "main" {
                        self.parse_main();
                    } else if self.peek_next_is_lparen() {
                        self.eat(&TokenType::Void);
                        self.parse_function();
                    } else {
                        self.eat(&TokenType::Void);
                        self.parse_declaration();
                    }
                }
                TokenType::Window => self.parse_window(),
                _ => self.parse_statement(),
            }
        }
    }

    fn parse_import(&mut self) {
        self.eat(&TokenType::Import);
        if self.current.token_type == TokenType::LAngle {
            self.eat(&TokenType::LAngle);
            self.eat(&TokenType::Identifier);
            if self.current.token_type == TokenType::Colon {
                self.eat(&TokenType::Colon);
                self.eat(&TokenType::Identifier);
            }
            self.eat(&TokenType::RAngle);
        } else if self.current.token_type == TokenType::StringLiteral {
            self.eat(&TokenType::StringLiteral);
        } else {
            self.eat(&TokenType::Identifier);
        }
        eprintln!(
            "DEBUG parse_import: finished, current token = {:?} ('{}')",
            self.current.token_type, self.current.lexeme
        );
    }

    fn parse_declaration(&mut self) {
        let var_type = self.parse_type();
        loop {
            let name = self.current.lexeme.clone();
            self.eat(&TokenType::Identifier);
            if self.current.token_type == TokenType::LBracket {
                self.eat(&TokenType::LBracket);
                let _size = self.current.value as i32;
                self.eat(&TokenType::Number);
                self.eat(&TokenType::RBracket);
                if self.current.token_type == TokenType::Assign {
                    self.eat(&TokenType::Assign);
                    self.eat(&TokenType::LBrace);
                    // Parse array init, for simplicity skip
                    while self.current.token_type != TokenType::RBrace {
                        if self.current.token_type == TokenType::Number {
                            self.eat(&TokenType::Number);
                        }
                        if self.current.token_type == TokenType::Comma {
                            self.eat(&TokenType::Comma);
                        } else {
                            break;
                        }
                    }
                    self.eat(&TokenType::RBrace);
                }
            }
            self.allocate_var(&name, var_type.clone());
            if self.current.token_type == TokenType::Comma {
                self.eat(&TokenType::Comma);
            } else {
                break;
            }
        }
        self.eat(&TokenType::Semicolon);
    }

    fn parse_function(&mut self) {
        let return_type = self.parse_type();
        let name = self.current.lexeme.clone();
        self.eat(&TokenType::Identifier);
        self.eat(&TokenType::LParen);
        let mut params = Vec::new();
        while self.current.token_type != TokenType::RParen {
            let param_type = self.parse_type();
            let param_name = self.current.lexeme.clone();
            self.eat(&TokenType::Identifier);
            params.push((param_name, param_type));
            if self.current.token_type == TokenType::Comma {
                self.eat(&TokenType::Comma);
            }
        }
        self.eat(&TokenType::RParen);
        self.eat(&TokenType::LBrace);
        let func = Function {
            name: name.clone(),
            return_type,
            params,
            start_label: self.new_label(),
            is_method: false,
        };
        self.functions.insert(name, func);
        self.parse_program_block();
        self.eat(&TokenType::RBrace);
    }

    fn parse_class(&mut self) {
        self.eat(&TokenType::Class);
        let class_name = self.current.lexeme.clone();
        self.eat(&TokenType::Identifier);
        self.eat(&TokenType::LBrace);
        let mut members = Vec::new();
        let mut methods = Vec::new();
        while self.current.token_type != TokenType::RBrace {
            if matches!(
                self.current.token_type,
                TokenType::Int
                    | TokenType::Double
                    | TokenType::String
                    | TokenType::Bool
                    | TokenType::Void
            ) {
                if self.peek_next_is_lparen() {
                    let method = self.parse_method(&class_name);
                    methods.push(method);
                } else {
                    let (name, typ) = self.parse_member();
                    members.push((name, typ));
                    self.eat(&TokenType::Semicolon);
                }
            } else {
                panic!("Unexpected in class");
            }
        }
        self.eat(&TokenType::RBrace);
        let class = Class {
            name: class_name.clone(),
            members,
            methods,
        };
        self.classes.insert(class_name, class);
    }

    fn parse_member(&mut self) -> (String, Type) {
        let typ = self.parse_type();
        let name = self.current.lexeme.clone();
        self.eat(&TokenType::Identifier);
        (name, typ)
    }

    fn parse_method(&mut self, class_name: &str) -> Function {
        let return_type = self.parse_type();
        let name = self.current.lexeme.clone();
        self.eat(&TokenType::Identifier);
        self.eat(&TokenType::LParen);
        let mut params = Vec::new();
        while self.current.token_type != TokenType::RParen {
            let param_type = self.parse_type();
            let param_name = self.current.lexeme.clone();
            self.eat(&TokenType::Identifier);
            params.push((param_name, param_type));
            if self.current.token_type == TokenType::Comma {
                self.eat(&TokenType::Comma);
            }
        }
        self.eat(&TokenType::RParen);
        self.eat(&TokenType::LBrace);
        let func = Function {
            name,
            return_type,
            params,
            start_label: self.new_label(),
            is_method: true,
        };
        self.functions
            .insert(format!("{}.{}", class_name, func.name), func.clone());
        self.parse_program_block();
        self.eat(&TokenType::RBrace);
        func
    }

    fn parse_window(&mut self) {
        self.eat(&TokenType::Window);
        self.eat(&TokenType::Identifier); // name
        self.eat(&TokenType::Identifier); // icon
        self.eat(&TokenType::Identifier); // theme
                                          // Skip dimensions for simplicity
        while self.current.token_type == TokenType::Number {
            self.eat(&TokenType::Number);
        }
        self.eat(&TokenType::LBrace);
        // For simplicity, skip window blocks
        self.parse_program_block();
        self.eat(&TokenType::RBrace);
    }

    fn parse_return(&mut self) {
        self.eat(&TokenType::Return);
        self.parse_expression();
        self.emit_ret();
        self.eat(&TokenType::Semicolon);
    }

    fn parse_try(&mut self) {
        self.eat(&TokenType::Try);
        self.eat(&TokenType::LBrace);
        self.parse_program_block();
        self.eat(&TokenType::RBrace);
        self.eat(&TokenType::Catch);
        self.eat(&TokenType::LBrace);
        self.parse_program_block();
        self.eat(&TokenType::RBrace);
        // For simplicity, no actual exception handling code generation
    }

    fn parse_throw(&mut self) {
        self.eat(&TokenType::Throw);
        self.parse_expression();
        self.eat(&TokenType::Semicolon);
        // For simplicity, just emit a placeholder
        self.emit_mov_rax_imm(0);
    }

    fn parse_assignment_or_call(&mut self) {
        let name = self.current.lexeme.clone();
        self.eat(&TokenType::Identifier);
        if self.current.token_type == TokenType::Assign {
            self.eat(&TokenType::Assign);
            self.parse_expression();
            if let Some(&(ref _var_type, offset)) = self.variables.get(&name) {
                self.emit_mov_rbp_offset_rax(offset);
            } else {
                panic!("Undefined variable: {}", name);
            }
            self.eat(&TokenType::Semicolon);
        } else if self.current.token_type == TokenType::PlusAssign {
            self.eat(&TokenType::PlusAssign);
            self.parse_expression();
            if let Some(&(ref _var_type, offset)) = self.variables.get(&name) {
                self.emit_pop_rbx();
                self.emit_mov_rax_rbp_offset(offset);
                self.emit_add_rax_rbx();
                self.emit_mov_rbp_offset_rax(offset);
            } else {
                panic!("Undefined variable: {}", name);
            }
            self.eat(&TokenType::Semicolon);
        } else if self.current.token_type == TokenType::LParen {
            self.eat(&TokenType::LParen);
            // Parse args
            while self.current.token_type != TokenType::RParen {
                self.parse_expression();
                if self.current.token_type == TokenType::Comma {
                    self.eat(&TokenType::Comma);
                }
            }
            self.eat(&TokenType::RParen);
            self.emit_call(&name);
            self.emit_push_rax();
            if self.current.token_type == TokenType::Semicolon {
                self.eat(&TokenType::Semicolon);
            }
        } else {
            panic!("Expected assignment or function call");
        }
    }

    fn peek_next_is_lparen(&mut self) -> bool {
        let next_type = self.peek_next_token_type();
        next_type == TokenType::LParen
    }

    fn peek_next_token_type(&self) -> TokenType {
        let mut temp_lexer = self.lexer.clone();
        temp_lexer.next_token().token_type
    }

    fn parse_statement(&mut self) {
        match self.current.token_type {
            TokenType::While => self.parse_while(),
            TokenType::For => self.parse_for(),
            TokenType::If => self.parse_if(),
            TokenType::Try => self.parse_try(),
            TokenType::Throw => self.parse_throw(),
            TokenType::Return => self.parse_return(),
            TokenType::Identifier => self.parse_assignment_or_call(),
            _ => {
                self.parse_expression();
                if self.current.token_type == TokenType::Semicolon {
                    self.eat(&TokenType::Semicolon);
                }
            }
        }
    }

    fn parse_for(&mut self) {
        eprintln!("parse_for: starting");
        self.eat(&TokenType::For);
        self.eat(&TokenType::LParen);
        eprintln!(
            "parse_for: after LParen, token = {:?}",
            self.current.token_type
        );

        let var_type = self.parse_type();
        let var_name = self.current.lexeme.clone();
        self.eat(&TokenType::Identifier);
        self.eat(&TokenType::Assign);
        self.parse_expression();
        self.eat(&TokenType::Semicolon);
        self.allocate_var(&var_name, var_type);

        if let Some(&(ref _var_type, offset)) = self.variables.get(&var_name) {
            self.emit_mov_rbp_offset_rax(offset);
        }

        let start_label = self.new_label();
        let end_label = self.new_label();

        self.emit_label(&start_label);

        self.parse_expression();
        self.emit_test_rax();
        self.emit_je(&end_label);

        self.eat(&TokenType::Semicolon);

        let update_name = self.current.lexeme.clone();
        self.eat(&TokenType::Identifier);
        self.eat(&TokenType::PlusAssign);
        self.parse_expression();

        if let Some(&(ref _var_type, offset)) = self.variables.get(&update_name) {
            self.emit_pop_rbx();
            self.emit_mov_rax_rbp_offset(offset);
            self.emit_add_rax_rbx();
            self.emit_mov_rbp_offset_rax(offset);
        }

        self.eat(&TokenType::RParen);
        self.eat(&TokenType::LBrace);
        self.parse_program_block();
        self.eat(&TokenType::RBrace);

        self.emit_jmp(&start_label);
        self.emit_label(&end_label);
    }

    fn parse_main(&mut self) {
        if self.current.token_type == TokenType::Void {
            self.eat(&TokenType::Void);
        }
        self.eat(&TokenType::Identifier);
        self.eat(&TokenType::LParen);
        self.eat(&TokenType::RParen);
        self.eat(&TokenType::LBrace);
        self.parse_program_block();
        self.eat(&TokenType::RBrace);
    }

    fn parse_assignment(&mut self) {
        let name = self.current.lexeme.clone();
        self.eat(&TokenType::Identifier);
        self.eat(&TokenType::Assign);
        self.parse_expression();
        if let Some(&(ref _var_type, offset)) = self.variables.get(&name) {
            self.emit_mov_rbp_offset_rax(offset);
        } else {
            panic!("Undefined variable: {}", name);
        }
    }

    fn parse_if(&mut self) {
        self.eat(&TokenType::If);
        let else_label = self.new_label();
        let end_label = self.new_label();
        self.parse_expression();
        self.emit_test_rax();
        self.emit_je(&else_label);
        self.eat(&TokenType::LBrace);
        self.parse_program_block();
        self.eat(&TokenType::RBrace);
        if self.current.token_type == TokenType::Else {
            self.eat(&TokenType::Else);
            self.emit_jmp(&end_label);
            self.emit_label(&else_label);
            self.eat(&TokenType::LBrace);
            self.parse_program_block();
            self.eat(&TokenType::RBrace);
            self.emit_label(&end_label);
        } else {
            self.emit_label(&else_label);
        }
    }

    fn parse_while(&mut self) {
        self.eat(&TokenType::While);
        let start_label = self.new_label();
        let end_label = self.new_label();
        self.emit_label(&start_label);
        self.parse_expression();
        self.emit_test_rax();
        self.emit_je(&end_label);
        self.eat(&TokenType::LBrace);
        self.parse_program_block();
        self.eat(&TokenType::RBrace);
        self.emit_jmp(&start_label);
        self.emit_label(&end_label);
    }

    fn parse_program_block(&mut self) {
        while self.current.token_type != TokenType::RBrace
            && self.current.token_type != TokenType::End
        {
            self.parse_statement();
        }
    }

    fn parse_expression(&mut self) {
        self.parse_term();
        while matches!(self.current.token_type, TokenType::Plus | TokenType::Minus) {
            let op = self.current.token_type.clone();
            self.eat(&op);
            self.parse_term();
            self.emit_pop_rbx();
            self.emit_pop_rax();
            match op {
                TokenType::Plus => self.emit_add_rax_rbx(),
                TokenType::Minus => self.emit_sub_rax_rbx(),
                _ => unreachable!(),
            }
            self.emit_push_rax();
        }
    }

    fn parse_term(&mut self) {
        self.parse_factor();
        while matches!(self.current.token_type, TokenType::Star | TokenType::Slash) {
            let op = self.current.token_type.clone();
            self.eat(&op);
            self.parse_factor();
            self.emit_pop_rbx();
            self.emit_pop_rax();
            match op {
                TokenType::Star => self.emit_mul_rbx(),
                TokenType::Slash => self.emit_div_rbx(),
                _ => unreachable!(),
            }
            self.emit_push_rax();
        }
    }

    fn parse_factor(&mut self) {
        match self.current.token_type {
            TokenType::Number => {
                let val = self.current.value;
                self.eat(&TokenType::Number);
                self.emit_mov_rax_imm(val);
                self.emit_push_rax();
            }
            TokenType::StringLiteral => {
                // For simplicity, treat as number for now
                self.eat(&TokenType::StringLiteral);
                self.emit_mov_rax_imm(0);
                self.emit_push_rax();
            }
            TokenType::New => {
                self.eat(&TokenType::New);
                let class_name = self.current.lexeme.clone();
                self.eat(&TokenType::Identifier);
                self.eat(&TokenType::LParen);
                self.eat(&TokenType::RParen);
                // For simplicity, allocate space and return pointer
                self.emit_mov_rax_imm(0); // placeholder
                self.emit_push_rax();
            }
            TokenType::Identifier => {
                let name = self.current.lexeme.clone();
                self.eat(&TokenType::Identifier);
                if self.current.token_type == TokenType::Dot {
                    self.eat(&TokenType::Dot);
                    let member = self.current.lexeme.clone();
                    self.eat(&TokenType::Identifier);
                    if self.current.token_type == TokenType::LParen {
                        // Method call
                        self.eat(&TokenType::LParen);
                        // Parse args
                        while self.current.token_type != TokenType::RParen {
                            self.parse_expression();
                            if self.current.token_type == TokenType::Comma {
                                self.eat(&TokenType::Comma);
                            }
                        }
                        self.eat(&TokenType::RParen);
                        self.emit_call(&format!("{}.{}", name, member));
                        self.emit_push_rax();
                    } else {
                        // Member access, for simplicity treat as variable
                        self.emit_mov_rax_imm(0); // placeholder
                        self.emit_push_rax();
                    }
                } else if self.current.token_type == TokenType::LParen {
                    // Function call, already handled in parse_assignment_or_call
                    panic!("Unexpected function call in factor");
                } else {
                    if let Some(&(ref _var_type, offset)) = self.variables.get(&name) {
                        self.emit_mov_rax_rbp_offset(offset);
                        self.emit_push_rax();
                    } else {
                        panic!("Undefined variable: {}", name);
                    }
                }
            }
            TokenType::LParen => {
                self.eat(&TokenType::LParen);
                self.parse_expression();
                self.eat(&TokenType::RParen);
            }
            _ => panic!("Expected factor"),
        }
    }

    fn parse_type(&mut self) -> Type {
        match self.current.token_type {
            TokenType::Int => {
                self.eat(&TokenType::Int);
                Type::Int
            }
            TokenType::Double => {
                self.eat(&TokenType::Double);
                Type::Double
            }
            TokenType::String => {
                self.eat(&TokenType::String);
                Type::String
            }
            TokenType::Bool => {
                self.eat(&TokenType::Bool);
                Type::Bool
            }
            TokenType::Void => {
                self.eat(&TokenType::Void);
                Type::Void
            }
            _ => panic!("Expected type"),
        }
    }

    fn allocate_var(&mut self, name: &str, var_type: Type) -> i32 {
        if self.variables.contains_key(name) {
            panic!("Variable already declared: {}", name);
        }
        self.stack_size += 8;
        self.variables
            .insert(name.to_string(), (var_type, self.stack_size));
        self.stack_size
    }
}
