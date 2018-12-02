NAME = npuzzle

EXEC_PATH = target/debug/npuzzle

RM = rm -f

DEPENDENCIES =	src/models/mod.rs\
		        src/parser/mod.rs\
		        src/solver/mod.rs\
		        src/solver/astar.rs\
		        src/solver/smart_astar.rs\
		        src/solver/common_functions.rs\
		        src/solver/solvability.rs\
		        src/heuristics/mod.rs\
		        src/output/mod.rs\
		        src/output/graphical/mod.rs\
		        src/output/graphical/display.rs\
		        src/output/graphical/interface.rs\
		        src/output/graphical/conditions.rs\
		        src/output/graphical/objects/mod.rs\
		        src/output/graphical/objects/cell_object.rs\
		        src/output/graphical/objects/rect.rs\
		        src/output/graphical/objects/text.rs\
		        src/output/terminal/mod.rs

all : $(EXEC_PATH) $(NAME)

$(EXEC_PATH) : $(DEPENDENCIES)
	cargo build

$(NAME) :
	ln -sf $(EXEC_PATH) $@

clean :
	$(RM) $(NAME)

fclean : clean
	cargo clean

re : fclean all

.PHONY: all clean fclean re
