use crate::lexer::{Lexer, Token, TokenType};
use std::collections::HashMap;

pub struct Parser {
    lexer: Lexer,
    current: Token,
    variables: HashMap<String, i32>,
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
            stack_size: 0,
            label_count: 0,
            labels: HashMap::new(),
            unresolved: HashMap::new(),
            code: Vec::new(),
        }
    }

    fn eat(&mut self, token_type: TokenType) {
        if self.current.token_type == token_type {
            self.current = self.lexer.next_token();
        } else {
            panic!("Parse error");
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
            self.parse_statement();
        }
    }

    fn parse_statement(&mut self) {
        match self.current.token_type {
            TokenType::Let => self.parse_let(),
            TokenType::While => self.parse_while(),
            TokenType::If => self.parse_if(),
            TokenType::Identifier => self.parse_assignment(),
            _ => self.parse_expression(),
        }
    }

    fn parse_let(&mut self) {
        self.eat(TokenType::Let);
        if let TokenType::Identifier = self.current.token_type {
            let name = self.current.lexeme.clone();
            self.eat(TokenType::Identifier);
            self.eat(TokenType::Assign);
            self.parse_expression();
            let offset = self.allocate_var(&name);
            self.emit_mov_rbp_offset_rax(offset);
        } else {
            panic!("Expected identifier");
        }
    }

    fn parse_assignment(&mut self) {
        let name = self.current.lexeme.clone();
        self.eat(TokenType::Identifier);
        self.eat(TokenType::Assign);
        self.parse_expression();
        if let Some(&offset) = self.variables.get(&name) {
            self.emit_mov_rbp_offset_rax(offset);
        } else {
            panic!("Undefined variable: {}", name);
        }
    }

    fn parse_if(&mut self) {
        self.eat(TokenType::If);
        let else_label = self.new_label();
        let end_label = self.new_label();
        self.parse_expression();
        self.emit_test_rax();
        self.emit_je(&else_label);
        self.eat(TokenType::LBrace);
        self.parse_program_block();
        self.eat(TokenType::RBrace);
        if self.current.token_type == TokenType::Else {
            self.eat(TokenType::Else);
            self.emit_jmp(&end_label);
            self.emit_label(&else_label);
            self.eat(TokenType::LBrace);
            self.parse_program_block();
            self.eat(TokenType::RBrace);
            self.emit_label(&end_label);
        } else {
            self.emit_label(&else_label);
        }
    }

    fn parse_while(&mut self) {
        self.eat(TokenType::While);
        let start_label = self.new_label();
        let end_label = self.new_label();
        self.emit_label(&start_label);
        self.parse_expression();
        self.emit_test_rax();
        self.emit_je(&end_label);
        self.eat(TokenType::LBrace);
        self.parse_program_block();
        self.eat(TokenType::RBrace);
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
            self.eat(op);
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
            self.eat(op);
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
                self.eat(TokenType::Number);
                self.emit_mov_rax_imm(val);
                self.emit_push_rax();
            }
            TokenType::Identifier => {
                let name = self.current.lexeme.clone();
                self.eat(TokenType::Identifier);
                if self.current.token_type == TokenType::LParen {
                    self.eat(TokenType::LParen);
                    self.eat(TokenType::RParen);
                    self.emit_call(&name);
                    self.emit_push_rax();
                } else {
                    if let Some(&offset) = self.variables.get(&name) {
                        self.emit_mov_rax_rbp_offset(offset);
                        self.emit_push_rax();
                    } else {
                        panic!("Undefined variable: {}", name);
                    }
                }
            }
            TokenType::LParen => {
                self.eat(TokenType::LParen);
                self.parse_expression();
                self.eat(TokenType::RParen);
            }
            _ => panic!("Expected factor"),
        }
    }

    fn allocate_var(&mut self, name: &str) -> i32 {
        if self.variables.contains_key(name) {
            panic!("Variable already declared: {}", name);
        }
        self.stack_size += 8;
        self.variables.insert(name.to_string(), self.stack_size);
        self.stack_size
    }
}
