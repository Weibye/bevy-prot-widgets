# Bevy-Prot-Widgets

Prototype widgets for [Bevy](https://github.com/bevyengine/bevy)!

Bevy is in need of _Widgets_ and higher levels of abstractions on top of the base UI functionality. 
But in order to understand the actual requirements, this project will strive to build widget-functionality starting at the lowest level first.
Then move on to identify commonalities and higher levels of abstractions that will make sense.


## Widgets

### Button Widget

**Description:** Allows user to trigger an action.

| Feature | Status |
|---|---|
| Enabled / Disabled | Yes |
| Trigger policy | Yes |
| Hover visuals | Yes |
| Press / Release visuals | Yes |
| Action mapping | No |
| Animations | No |

**Types:**
- Text button
- Icon button
- Label button (Button with icon + text)

[Reference](https://mui.com/material-ui/react-button/#basic-button)

### Button group

**Planned, not yet supported.**

### Icon Widget

- Simply displays an icon.
- Should be able to be used anywhere.

https://material.angular.io/components/icon/overview

### Text Widget

- Shows a single line of text.
- Control if user should be allowed to mark the text or not
- Can the user focus on this widget?
- Rich text support

**Planned, not yet supported.**

### Text input widget

**Planned, not yet supported.**


### Checkboxes

**Planned, some support.**

- Checkbox
- Checkbox with label

- Inteterminate state (minus sign)
    - User can never put it in indererminate state
    - Only exist as a computed value

### Radio Button

**Planned, not yet supported.**


### Radio Group

**Planned, not yet supported.**

- Controls that only one of the radio-buttons in the group is selected at any given time
- Needs to be set with an initial value

### Progress Bar
- Determinate (0 -> 1 progress)
- Indeterminate (unknown progress, just show loading animation)
- Direction of flow (from what direction is it being filled up)
- Can also be used for health-bars

https://material.angular.io/components/progress-bar/overview

### Progress Spinner
- Determinate
- Indeterminate

https://material.angular.io/components/progress-spinner/overview

### Slide Toggle

https://material.angular.io/components/slide-toggle/overview

### Slider

https://material.angular.io/components/slider/overview


### Tooltip widget

## Features needed

- Background color
- Outline
- Rounded borders
- (nice to have) drop shadow