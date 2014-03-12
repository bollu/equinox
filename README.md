Equniox
=======

A project to experiment with Rust, Carmack's ideas and, well, build a game.

Architecture (hah!)
====================

Basic idea is to have a set of "pumps" which receive input and use this input to produce output.

Examples of such pumps:

	RenderPump: takes drawables, renders to screen
	EventPump: takes events, passes them to eventHandlers
	AudioPump: takes audio, plays it.

The game is (supposed to be) component based, but I'm sure that I'll have to bend the rules by quite a bit to fit Rust's model. However, the language seems to have an ability to lead to good design (unlink C++ by the way), so it should be clean - if unorthodox. 

The major idea behind this is to apply John Carmack's idea of a purely functional game world - the previous state of the game is fed to the objects, which generate the next state of the game. The differentiating factor is that the previous state is *immutable* - which is a heavy restriction to work with. However, this *technically* makes the engine fully parallel precisely because of this immutability. 

It should be interesting too see how Carmack's ideas will influence the design of the engine - I assume quite a few things (including physics) will have to be done differently. It should be interesting - if not useful and practical.


Rules of this project
=====================

* All art is procedurally generated. No exceptions. This restriction is the most interesting one, since it puts a direct upper bound on complexity.

Due to this restriction, I plan on using a limbo-y look to the game - black shadows on a white background,
using color to accentuate and differentiate objects.

* The game (should) be a rougelite hack-n-slash with a sidescrolling perspective. it should be interesting due to a few key features:

Dynamic Lighting - the game will feature dynamic lighting [like this](http://archive.gamedev.net/archive/reference/programming/features/2dsoftshadow/) and will have an impact on gameplay, not just aesthetics.

Procedural generation - everyone seems to be throwing this in these days, and it's fun to do, so why not? ;)

WEAPONS! - the hope is to have randomly generated weapons with lots and lots of modifiers - kind of how Bulletstorm does it. 


Final Thoughts:
===============

This project will quite certainly __never__ be completed - however, this design document outlines my thought process and hopefully the game should reach an acceptable level of complexity, even if I do not finish it. 

I hope everyone who decides to read the code of this project learns something - either how to do something or (more often than not) how *not* to do something. Both of these should be enlightening :)

Let's hope I finish this project!