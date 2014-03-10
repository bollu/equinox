mod State;
use rsfml::graphics::Text;

struct  MainMenu {
	menuOpts: ~[rsfml::graphics::Text],
	currentIndex: uint,
}