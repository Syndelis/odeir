CC=g++ -std=c++20
CFLAGS=-g -fpermissive
CFLAGS_LIBS=-c

ODEIR_INCS=-I../include/ -I./include/
ODEIR_LIB=../target/debug/libodeir.a

CATCH2_PATH=lib/catch2
CATCH2_EXTRAS_PATH=$(CATCH2_PATH)/extras
CATCH2_EXTRAS_SRC=$(wildcard $(CATCH2_EXTRAS_PATH)/*.cpp)
CATCH2_EXTRAS_OBK=$(foreach src,$(CATCH2_EXTRAS_SRC),$(OBJ_DIR)/$(notdir $(basename $(src))).o)


INCLUDE_DIRS=-I./ -I../ $(ODEIR_INCS) -I$(CATCH2_EXTRAS_PATH)

LINK_DIRS=
LINKS=
DEFINE=

FIXTURES_DIR=fixtures
FIXTURES_SRC=$(wildcard $(FIXTURES_DIR)/*.json)
FIXTURES=$(foreach src,$(FIXTURES_SRC),$(FIXTURES_DIR)/$(notdir $(basename $(src))).h)

OBJ_DIR=obj
TESTS_SRC=$(wildcard src/**.cpp)
TESTS_OBJ=$(foreach src,$(TESTS_SRC),$(OBJ_DIR)/$(notdir $(basename $(src))).o)
OBJ=$(TESTS_OBJ) $(CATCH2_EXTRAS_OBK)
EXE=test

VPATH=src $(wildcard src/*) $(CATCH2_EXTRAS_PATH) $(ODEIR_LIB)

run: build
	./$(EXE)

valgrind: build
	valgrind --leak-check=full --show-leak-kinds=all --undef-value-errors=no --error-exitcode=1 ./$(EXE)

build: $(EXE)

$(EXE): $(FIXTURES) $(ODEIR_LIB) $(OBJ_DIR) $(OBJ)
	$(CC) $(CFLAGS) $(OBJ) $(ODEIR_LIB) -o $@ $(INCLUDE_DIRS) $(LINK_DIRS) $(LINKS) $(DEFINE)

$(FIXTURES): $(FIXTURES_DIR)/%.h : $(FIXTURES_DIR)/%.json
	xxd -i $< > $@

$(ODEIR_LIB):
	cargo +nightly build

$(OBJ_DIR):
	mkdir -p $@

$(OBJ): $(OBJ_DIR)/%.o : %.cpp
	$(CC) $(CFLAGS) $(CFLAGS_LIBS) $< -o $@ $(INCLUDE_DIRS) $(LINK_DIRS) $(LINKS) $(DEFINE)

clean:
	rm -f $(TESTS_OBJ) $(FIXTURES) $(EXE)

full-clean: clean
	rm -rf $(OBJ_DIR)
