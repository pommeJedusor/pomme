# BinaryWorld

## the goal

The goal of this project is to developp a deep understanding of how computers work at the lowest level.
This project will be inspire of people having create computers inside games like minecraft and others.
I'm gonna do it a bit differently, I'll create my own 'game' which will be very simple it will be a 2D grid
made of cells

## the rules

each cell will have four main parameters
1. value
this one is simply how much 'on' cells are powering the cell and it goes from 0 to 3 included
e.g.

|  |  |  |  |  |
|--|--|--|--|--|
|A |\<|B |\>|C |
|^ |  |- |  |- |
|D |\||0 |\||0 |

here will suppose that each '0' cell is off and each other is on  
D and B are powering A so A has a value of 2  
nobody is powering D nor B so D and B have a value of 0  
B is powering C so C has a value of 1  

2. requirements
this one is a table to know if the cell is on relatively to its value
e.g.
A has requirements of 0b1111
so it will be 'on' whatever its value is  
B has requirements of 0b0001
so it will be 'on' if and only if its value is 0  
C has requirements of 0b0010
so it will be 'on' if and only if its value is 1 (perfect for xor operations)  
D has requirements of 0b0100
so it will be 'on' if and only if its value is 2 (perfect for and operations)  
E has requirements of 0b0110
so it will be 'on' if and only if its value is 2 or 1 (perfect for or operations)  

3. is_storage
this one is to know if it's a normal block or a storage block which have a totaly different structure and goal
I'll describe those blocks later

4. is_on
this one is simply to know if the block is on or off
I don't strictly need it but the previous datas needed 7 bits so I add it
because the last bit would have been unused anyway


### and
|   |   |   |
|---|---|---|
|0  |\| |A  |
|\- |   |v  |  
|B  |>  |C  |


0 value_required = 0000  
A value_required = 0010  
B value_required = 0010  
C value_required = 0100  

### or  
|   |   |   |
|---|---|---|
|0  |\| |A  |
|\- |   |v  |  
|B  |>  |C  |


0 value_required = 0000  
A value_required = 0010  
B value_required = 0010  
C value_required = 0110  

### xor
|   |   |   |
|---|---|---|
|0  |\| |A  |
|\- |   |v  |  
|B  |>  |C  |


0 value_required = 0000  
A value_required = 0010  
B value_required = 0010  
C value_required = 0010  

### nand
|   |   |   |
|---|---|---|
|0  |\| |A  |
|\- |   |v  |  
|B  |>  |C  |


0 value_required = 0000  
A value_required = 0010  
B value_required = 0010  
C value_required = 0011  



### storing

to store bit values there is storing block whose blocks start with a '^' as a convention
and must be connect to two cells
1. one with a '>' to get the value from  
2. one with a '-' horizontally or '|' vertically to know if it should take the value (can only have one)  

|    |    |    |
|----|----|----|
|AA  |\>  |^A  |
|\-\-|\-\-|\|\||  
|00  |\|  |AB  |


^A will get the on/off of AA if and only if AB is on
elsewise it will keep it's current on/off (0 by default)

^A AA AB ^A  
0  0  0  0  
0  1  0  0  
0  0  1  0  
0  1  1  1  
1  0  0  1  
1  1  0  1  
1  0  1  0  
1  1  1  1  

first ^A previous ^A on/off  
AA AA on/off  
AB AB on/off  
last ^A new value of ^A  

### portals
to make complex structures possible when parts need to be able to
interact with each other without being block by each other,
there is portals
they don't work with storage blocks if the connection is '|'
you just need to write the name of the cell in lowercase and the power will be directly send to the cell
avoid putting to much power on one cell, don't forget it can't hold a value higher than 3

simple binary or

A->H 8 bits input a  
I->P 8 bits input b  
Q->X 8 bits value_required 0110 output c  

|   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|A  |\| |B  |\| |C  |\| |D  |\| |E  |\| |F  |\| |G  |\| |H  |\| |I  |\| |J  |\| |K  |\| |L  |\| |M  |\| |N  |\| |O  |\| |P  |
|v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |
|q  |\| |r  |\| |s  |\| |t  |\| |u  |\| |v  |\| |w  |\| |x  |\| |Q  |\| |R  |\| |S  |\| |T  |\| |U  |\| |V  |\| |W  |\| |X  |


the on/off of A go to q  
q being a portal it goes to Q  
the on/off of I go to q  
Q having a value required of 0110 will only be on/off if it get one or two of power  
so if A or I is true Q will be  
it is the same for the 7 others bits  


if we would to want it to store it:

A->H 8 bits input a  
I->P 8 bits input b  
Q->X 8 bits value_required 0110 output c  

|   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|A  |\| |B  |\| |C  |\| |D  |\| |E  |\| |F  |\| |G  |\| |H  |\| |I  |\| |J  |\| |K  |\| |L  |\| |M  |\| |N  |\| |O  |\| |P  |
|v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |
|q  |\| |r  |\| |s  |\| |t  |\| |u  |\| |v  |\| |w  |\| |x  |\| |Q  |\| |R  |\| |S  |\| |T  |\| |U  |\| |V  |\| |W  |\| |X  |
|\- |   |\- |   |\- |   |\- |   |\- |   |\- |   |\- |   |\- |   |\| |   |\| |   |\| |   |\| |   |\| |   |\| |   |\| |   |\| |
|0  |\| |0  |\| |0  |\| |0  |\| |0  |\| |0  |\| |0  |\| |0  |\| |Z  |\| |Z  |\| |Z  |\| |Z  |\| |Z  |\| |Z  |\| |Z  |\| |Z  |


if Z is true then the result will be stored otherwise it won't move

the problem now is that it is hard to access those result values
there is multiple solutions, I will show two of them
the first one is to make the second input go through a portal too
the second one is clone
like I did we the Z, if two cells have the same name, they are the same cell
so we could just do that:

|   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|A  |\| |B  |\| |C  |\| |D  |\| |E  |\| |F  |\| |G  |\| |H  |\| |I  |\| |J  |\| |K  |\| |L  |\| |M  |\| |N  |\| |O  |\| |P  |
|v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |v  |
|q  |\| |r  |\| |s  |\| |t  |\| |u  |\| |v  |\| |w  |\| |x  |\| |Q  |\| |R  |\| |S  |\| |T  |\| |U  |\| |V  |\| |W  |\| |X  |
|\- |   |\- |   |\- |   |\- |   |\- |   |\- |   |\- |   |\- |   |\| |   |\| |   |\| |   |\| |   |\| |   |\| |   |\| |   |\| |
|0  |\| |0  |\| |0  |\| |0  |\| |0  |\| |0  |\| |0  |\| |0  |\| |Z  |\| |Z  |\| |Z  |\| |Z  |\| |Z  |\| |Z  |\| |Z  |\| |Z  |
|\- |   |\- |   |\- |   |\- |   |\- |   |\- |   |\- |   |\- |   |\- |   |\- |   |\- |   |\- |   |\- |   |\- |   |\- |   |\- |
|Q  |\| |R  |\| |S  |\| |T  |\| |U  |\| |V  |\| |W  |\| |X  |0  |0  |0  |0  |0  |0  |0  |0  |0  |0  |0  |0  |0  |0  |0  |0  |
