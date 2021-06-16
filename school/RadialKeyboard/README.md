# RadialKeyboard

## INFO 463 Research Project

### Description
This keyboard was designed for devices the size of the apple watch (~42 mm) utilizing touch screen capabilities. The prototype was coded in Java utilizing Android Studio to generate an Android Application which simulates the desired functionality of the keyboard.


### Goal
The design goal for this application was to generate a system that would be faster than trying to implement a miniaturized QWERTY
keyboard. This input technique would be used on devices similar to the Apple Watch or Android Watches where the screens are
rectangular and too small for traditional input systems.

### Design Summary
We examined projects like ZoomBoard and tried to generate a system that would be engaging and fast to use once learned. After some
ideation we structured our design in a radial fashion because it equalized the distance between all keys. This minimized the issue
of need to access two keys consecutively but having them be on the opposite ends of a long keyboard layout.

We created a system where the main access keys would be arranged around the keyboard for immediate access. Only nine of these keys
could fit on the board so we came up with a strategy that involved a branching mechanic where each key could
lead to three different choices. While no character is selected, dragging over a button would lead to the button being in the held
down state, while the buttons to the left and right would alter to reflect the other two choices associated with that button.
By changing what the left and right buttons represent, we are able to improve through Fitt's Law by increasing the size
of the target, making input time shorter. Additionally, the held down state decreases movement time by not requiring the user
to raise their finger. Raising one's finger adds more time while the finger could be moving towards the next desired button.

The layout of the keyboard also went through various versions, with the two main choices being alphabetical lettering arranged
in a clockwise fashion or a letter frequency prioritized version where the letters most used in the english language were arranged
towards the center.

Symbol keys and numerical keys can be accessed by swiping left or right. This system scrolls through the three different keyboards
which all operate on the same layout. The pages also wrap around so you can continuously scroll to access each page if desired.

### Testing
We had various users test the system by entering in a collection of sentences from a bank of inputs. The users were able to play
around with the input technique for a few minutes before being administered the test.


### Test Feedback
While the test subjects agreed that the input system would be fast once the user learned how to use the system, the initial learning
curve would be difficult to get over. This stemmed from the smoothness of our test application as well as the unique layout that we
pursued with this design. Many users felt frustrated because actuation of a button sometimes would not occur when they wanted (combination of the fat finger problem as well as an implementation issue). Furthermore, it was noted that there was a significant
slowdown towards the beginning of each test while the user attempted to figure out where some of the keys were. 

### Conclusion
The combination of these issues might outweigh the eventual usefulness of this system if not solved because users would avoid using a frustrating system. However, for an experienced user the design can be incredibly fast as a method of text input on a small device. Whether or not someone would want to use it considering its steep learning curve is a further design challenge that this project will need to address.

### Contributors
#### Designers
- Aisha Toulegenova
- Derek Wang
- Sybil Wang

#### Developers
- Derek Wang

### Afterwards
Design work done for INFO 463 (Inputs and Interactions) at the Univeristy of Washington
