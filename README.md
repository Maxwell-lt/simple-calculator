# Simple Calculator

Calculator application supporting basic arithmetic operations, built with Rust and the Iced GUI library.

## Design plan

* Standard calculator layout including at least these buttons:
    * Digits 0-9
    * Decimal separator
    * Add, Subtract, Multiply, Divide
    * Equals
    * Clear Entry, All Clear
* Text display showing a single number
* Behavior like that of a basic physical calculator:
    * When turning the calculator on, the display shows 0
    * Pressing digit keys enters a number into the display, with each press adding a new digit on the smaller end of the number
    * Pressing an operation key after a number is entered silently puts the calculator into that mode
    * After an operation key is pressed, using the digit keys enters a new number
    * Pressing Equals after a number, an operation key, and another number is entered will perform the calculator and show the result
    * As in the prior line, if an operation key is pressed instead of Equals, the prior calculation will be performed and the result shown, but the calculator will also be ready to accept another input which will be processed against the result now shown
        * i.e. if the sequence `4 + 5 +` is entered, the screen will show `9`. If the number `6` is then entered, then after the next press of Equals or an operation key, the number 6 will be added to that result, and `15` will be shown
    * Entering a number after the Equals key is pressed will begin a new calculation
    * Pressing Equals after the Equals key is pressed will re-run the latest calculation
        * i.e. if the sequence `4 + 5 =` is entered, the screen will show `9`. If `=` is entered again, the number 5 will be added to that result, and `14` will be shown
    * Pressing the All Clear button will clear the active calculation and return the screen to the starting state
    * Pressing the Clear Entry button will clear only the current active number. If the sequence `4 + 5` is entered, then Clear Entry is pressed, the calculator will show `0`, and typing another number and pressing Equals will add it to `4`

Application will be split into two main components. The UI will be handled by Iced, which will hold an instance of a Calculator object and send button presses to it without performing any processing of its own. The Iced UI will also read the current screen value from the Calculator on each refresh of the view.
The Calculator object will keep track of its state, perform calculations when necessary, and provide an output with the current value that should be displayed. Tests will be written against this Calculator object.

