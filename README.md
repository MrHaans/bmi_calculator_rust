# BMI Calculator

## Overview

This application is a simple BMI (Body Mass Index) calculator built using Rust and the `eframe` crate. It provides a graphical user interface where users can input their height and weight to calculate and display their BMI along with their weight status.

## Libraries Used

- **`eframe`**: A framework for building GUI applications in Rust using the `egui` library. It provides tools for creating user interfaces and managing application state.

## How It Works

### Main Components

- **`BMIApp` Struct**: 
  - Holds the state of the application, including user inputs (`height_input` and `weight_input`), the BMI result (`bmi_result`), and the weight status (`status`).

- **`eframe::App` Implementation**:
  - **`update` Method**: The core of the application, responsible for rendering the user interface and handling user interactions. 

### UI Layout and Functionality

- **Central Panel**: The main panel of the application is set with a white background using `Frame`.
- **Vertical Centered Layout**: The layout is centered vertically on the screen.
- **Heading**: Displays the title "BMI Calculator" in a large, bold, black font.
- **Input Fields**: 
  - Users enter their height (in cm) and weight (in kg) using `TextEdit` fields.
  - The input fields are accompanied by labels instructing users on what to enter.
- **Calculate Button**: When clicked, it triggers the BMI calculation.
  - Height is converted from cm to meters.
  - BMI is calculated using the formula: `weight / (height * height)`.
  - The result and weight status (Underweight, Normal weight, Overweight, Obesity) are displayed based on the calculated BMI.
  - If the input values are invalid, an error message is shown.
- **Results Display**: The calculated BMI and corresponding status are displayed in black text below the calculation button.

## Concepts Used

- **State Management**: The application uses a struct (`BMIApp`) to manage and store the state of user inputs and calculation results.
- **Event Handling**: User interactions, such as clicking the "Calculate BMI" button, trigger updates in the application state and UI.
- **UI Layout**: Uses `egui`'s layout system to position and style UI elements.
- **Conditional Logic**: Determines the weight status based on the calculated BMI and displays appropriate messages.
- **Error Handling**: Handles invalid inputs by providing default values and error messages.

## Running the Application

To run this application, you need to have Rust and Cargo installed on your machine. Clone the repository and use Cargo to build and run the application:

```bash
git clone <repository-url>
cd <repository-directory>
cargo run
