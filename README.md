# P.O.M.M.E (Procedure oriented machine maker engine)

## the goal

The goal of this project is to developp a deep understanding of how computers work at the lowest level.
This project is inspired by people having create computers inside games like minecraft, Terraria and others.
I'm gonna do it a bit differently, I'll create my own 'game' which will be very simple and focused on that purpose which will make it a lot easier to build, test and debug the machines, which are known to be big struggles with normal games, by using the pomme language and a component system

## important concepts
in this section I'll explain basically what does the program do regardless of the language
so first of, it is a [graph](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)) more precisely a [directed](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)#Directed_graph) one made of two types of nodes: the *logical blocks* and the *storing blocks*, the former allow logic bitwise operations like *and*, *or* and *xor*. the latter will have as only purpose to store data you can see it at the ram or the ssd of a machine

### logical blocks

each nodes has two states either **on** or **off** which is dependent on other nodes states
so let's call a node *A* which have 'requirements' of 2 which means it will be **on** if and only if there is two nodes that are connected directly toward *A* that are **on**

a given node may have multiple requirements like 1 and 2 which will be the equivalent of a binary **or**
they are two special kinds of blocks which are **lamps** and **rocks**

lamps have 0, 1, 2, 3 and 4 as requirements which means that whatever the states of the other nodes they are **always on** and

rocks as the opposite don't have any requirements and are **always off**, their purpose is to be changed by user input during the execution of the program by switching from rock to lamp or from lamp to rock

### storing blocks

their purpose is to store data during the execution of the program, you can see it as a kind of ram
each of those nodes will have two nodes toward them
- button, if the button node is **off** then the storing block will keep its value regardless of everything else however if it's **on** it will **always** have the same value as its source node
- source, as said previously this is the node from which the storing block will get its value from if and only if the button node is **on**

## the pomme compiled language

this language has the purpose of being easily understand by the program without having much to think about

though you'll probably use more the pomme language directly it is interesting to understand the compiled version first to better understand why things are how they are

files of this kind are store with the pc extension for pomme compiled
```pc
1 00000 3 4
2 00000 3 4
3 00100 5
4 00110 5
^5 3 4
```
now I'll explain the previous lines, the '1' is simply the index of the node, you could start at whatever you want like 42 for example

the '00000' is the binary representation of the requirements the last zero being a boolean representing whether it should be **on** or **off** if there is zero node connected toward it that are **on**  

the 4th is the same but if there is one and only one 

the 3rd is the same if there is two and only two, you got the point

the '3 4' space separated digits are the connection it means there that the node one is connected towards the node 3 and 4

to explain the example with what we saw the node 3 will only be on if the node 1 and 2 are on which means it's a binary 'and'

whereas the node 4 will be on if there is at least one of them that is on which means that it is a binary 'or'

the node 5 is a storing block because it starts with a '^' and has as button node the node 3 and has as source the node 4, here it doesn't have any 'children' nodes but you can add the same way you would do for the logical blocks

## the pomme language
this language is still under development so due to the high likely hood of big changes I'll wait until it becomes stable to make that part of the readme but you can still look in the components folder to find some example with the extension .pomme and try to understand how they work, you can also compare them with their compiled version in .pc
