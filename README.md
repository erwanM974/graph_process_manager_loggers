# Generic loggers for graph_process_manager_core

4 types of loggers to log processes implemented/tooled with graph_process_manager_core :
- a GraphViz logger to produce graphical representations of the processes
- a nodesPrint logger to print in files intermediate objects build during the process
- a stepsTrace logger to print in files traces i.e. sequences of steps taken during the process
- a NFAIT logger to build a Non Deterministic Finite Automaton (with Immediate Transitions)

An example use for a trivial process representing the Fibonacci sequence is given in the test module.
A maximum depth is set to 10 for this example.

Applying a simple GraphViz logger to this example process gives the following representation:

<img src="./README_images/proc_fibo.svg" alt="GraphViz log of Fibonacci process">

The following Automata can be build from this exploration :

<img src="./README_images/fib_NFAIT.svg" alt="NFAIT of Fibonacci process">


A nodesPrint logger is used here to print the numbers in the Fibonacci sequence in text files
and a stepsTrace logger is used to print the sequence of 'next' steps 
(here it is a trivial application but for other processes there may be various kinds of steps etc.).

Below is represented the files generated by the two loggers for the example toy process:
<img src="./README_images/fibo_example.png" alt="files generated by the 2 loggers">


