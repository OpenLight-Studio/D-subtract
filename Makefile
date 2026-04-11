# Makefile
CXX = g++
CXXFLAGS = -std=c++17 -O2 -Wall -Wextra

dsubtract: main.cpp
	mkdir -p ./build/
	$(CXX) $(CXXFLAGS) -o ./build/$@ $<

asm: main.cpp
	$(CXX) -S $(CXXFLAGS) -o dsubtract.s $<

dsubtract-asm: dsubtract.s
	$(CXX) -o dsubtract-asm $< -lstdc++

clean:
	rm -f dsubtract dsubtract.s dsubtract-asm

.PHONY: clean asm dsubtract-asm
