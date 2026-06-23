# Compiler settings
CXX = g++
CXXFLAGS = -std=c++17 -Wall
LIBS = -lpthread

# Final executable name
TARGET = aura_engine

# Source files
SOURCES = main.cpp AuraEngine.cpp Telemetry.cpp
OBJECTS = $(SOURCES:.cpp=.o)

# Default target
all: $(TARGET)

$(TARGET): $(OBJECTS)
	$(CXX) $(OBJECTS) -o $(TARGET) $(LIBS)

# Compile sources
%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

# Clean up
clean:
	rm -f $(OBJECTS) $(TARGET)
