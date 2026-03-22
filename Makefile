# Makefile
CXX = g++
CXXFLAGS = -std=c++17 -O2 -Wall -Wextra

dsubtract: main.cpp
	mkdir ./build/
	$(CXX) $(CXXFLAGS) -o ./build/$@ $<

clean:
	rm -f dsubtract

.PHONY: clean
