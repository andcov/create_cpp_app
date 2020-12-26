# the compiler
CC = g++

# compiler flags:
#  -Wall turns on most, but not all, compiler warnings
CFLAGS  = -Wall

# the build target executable:
TARGET = main

all: $(TARGET)
	$(CC) $(CFLAGS) -o $(TARGET) $(TARGET).cpp && ./$(TARGET)

clean:
	$(RM) $(TARGET)