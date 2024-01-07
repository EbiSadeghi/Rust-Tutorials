Transcript: Hello Hello,

** Today we are going to go over the basics of loops in Rust.

Loops allow us to rerun certain parts of code repeatedly. We can use them to specify how many times we need to repeat the defined steps.

Without loops our code executes the tasks we define, but then comes to an end. Loops are helpful for keeping our program running until we need it to end,

So let's start with a simple loop. this will run forever, unless we write break; So let's see that.

So now that we want it to stop, let's put a break statement.

So here, let's look at while. It continues until a condition fails. So here let's define an incrementor i. We will set this to zero initially. Now let's put the condition to be that the loop will continue so long as the the incrementor is less than 10. So everytime we reach the end of the loop, let's increase i by one, and let's print out the value of i.

Now a helpful keyword here is continue, which we can use to skip certain areas. So here, let's adjust the loop to skip every even number. We can calculate the even numbers by seeing if the iterator modulo 2 is zero.

Now we can also nest our loops. So consider that we have two dogs, one of them is sneezing, and the other is running around in circles. Here's an example. So here, let's write this scenario out in code. We'll put the while loop for the dog running around, with i as the incrementor. And we'll put another while loop with the dog sneezing with j as the incrementor. And I've added a sleep here to slow down the code enough for us to play them side by side.

The last loop is a for loop, which iterates over a data structure. We'll discuss data structures more later on, but just know that we are storing a series of numbers here. And we'll increment over the array like so. One thing to note is that it is better to write your incrementor as ii instead of just i, because otherwise if you CTRL-F through the code in search of "i", the entire code base will light up. So it looks like I put a colon instead of a semicolon, so let's fix that. And we can see we get the average, which is 3.5.

Lastly, I just want to leave you with this meme. So a programmer's wife asks of him "While you're out, buy some milk", and he never comes back. Always remember to properly terminate your loops.
