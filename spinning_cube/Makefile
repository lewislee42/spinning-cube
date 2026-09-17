

CXX = g++
CXX_FLAGS = -std=c++20 -o2 -Wall -Wextra -Werror

SRC_FILES = main.cpp

TARGET = spinning_cube

all:
	@$(CXX) $(CXX_FLAGS) $(SRC_FILES) -o $(TARGET)

clean:
	@echo "\033[0;31mremoving files: \033[0m" $(TARGET)
	@rm -rf $(TARGET)
