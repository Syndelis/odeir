CC=g++ -std=c++20
CFLAGS=-fpermissive
CFLAGS_LIBS=-c

ODEIR_LIB=../target/debug/libodeir.a

CATCH2_PATH=lib/catch2
CATCH2_EXTRAS_PATH=$(CATCH2_PATH)/extras
CATCH2_EXTRAS_SRC=$(wildcard $(CATCH2_EXTRAS_PATH)/*.cpp)

INCLUDE_DIRS=-I./ -I../ -I../include/ -I$(CATCH2_EXTRAS_PATH)

LINK_DIRS=
LINKS=
DEFINE=

FIXTURES_DIR=fixtures
FIXTURES_SRC=$(wildcard $(FIXTURES_DIR)/*.json)
FIXTURES=$(foreach src,$(FIXTURES_SRC),$(FIXTURES_DIR)/$(notdir $(basename $(src))).h)

OBJ_DIR=obj
SRC=$(wildcard src/**.cpp) $(CATCH2_EXTRAS_SRC)
OBJ=$(foreach src,$(SRC),$(OBJ_DIR)/$(notdir $(basename $(src))).o)
EXE=test

VPATH=src $(wildcard src/*) $(CATCH2_EXTRAS_PATH)

run: all
	./$(EXE)

all: $(EXE)

$(EXE): $(FIXTURES) $(ODEIR_LIB) $(OBJ_DIR) $(OBJ)
	$(CC) $(CFLAGS) $(OBJ) $(ODEIR_LIB) -o $@ $(INCLUDE_DIRS) $(LINK_DIRS) $(LINKS) $(DEFINE)

$(FIXTURES): $(FIXTURES_DIR)/%.h : $(FIXTURES_DIR)/%.json
	xxd -i $< > $@

$(ODEIR_LIB):
	cargo build

$(OBJ_DIR):
	mkdir -p $@

$(OBJ): $(OBJ_DIR)/%.o : %.cpp
	$(CC) $(CFLAGS) $(CFLAGS_LIBS) $< -o $@ $(INCLUDE_DIRS) $(LINK_DIRS) $(LINKS) $(DEFINE)

clean:
	rm -rf $(OBJ_DIR) $(FIXTURES) $(EXE)
