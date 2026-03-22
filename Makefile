# Makefile
CXX = g++
CXXFLAGS = -std=c++17 -O2 -Wall -Wextra

dsubtract: main.cpp
	$(CXX) $(CXXFLAGS) -o $@ $<

clean:
	rm -f dsubtract

.PHONY: clean
