# Exploring Widgets in Bevy

WARNING: This is currently a work in progress.

The purpose of this repo is to explore designs and posibilites when it comes to UI Widgets in [Bevy](https://github.com/bevyengine/bevy).

At the moment, the goal is to establish the required low-level functionality that can be built on top of later.

## Widgets

### Button Widget

A button is pressed and fired once.
It will not be toggled on or off.

Status: Not supported

- Text button
    - Text
    - Contained
    - Outlined
- Button with icon _and_ text
- Icon button
    - Single button with an icon


Button Visual Behaviour


[Reference](https://mui.com/material-ui/react-button/#basic-button)

### Button group

Status: Not supported

### Text Widget

- Shows a single line of text.
- Control if user should be allowed to mark the text or not
- Can the user focus on this widget?
- Rich text support

### Text input widget

- 



### Checkboxes

Status: Not supported

- Checkbox
- Checkbox with label

- Inteterminate state (minus sign)
    - User can never put it in indererminate state
    - Only exist as a computed value

### Radio Button

Status: Not supported


### Radio Group

Status: Not supported

- Controls that only one of the radio-buttons in the group is selected at any given time
- Needs to be set with an initial value
- 


## Features needed

- Background color
- Outline
- Rounded borders
- (nice to have) drop shadow