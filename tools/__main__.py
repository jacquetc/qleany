import sys
from generator.qleany_generator_gui import main as gui_main

def main():
    
    if len(sys.argv) > 1 and sys.argv[1] == "gui":
        gui_main()
    else:
        gui_main()
        
        

if __name__ == "__main__":
    main()
