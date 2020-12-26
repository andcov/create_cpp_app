# the compiler
CC = g++

# compiler flags:
#  -Wall turns on most, but not all, compiler warnings
CFLAGS  = -Wall

# the build target executable:
TARGET = main

# I/O files
INPUT = input.txt
OUTPUT = output.txt

all: $(TARGET)
	$(CC) $(CFLAGS) -o $(TARGET) $(TARGET).cpp && ./$(TARGET)

io:
	touch $(INPUT) && touch $(OUTPUT)

clean:
	$(RM) $(TARGET)