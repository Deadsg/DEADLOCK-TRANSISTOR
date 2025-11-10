
import torch
import os
from Reasoning_DQN import DQNAgent

# Define parameters used by the model
STATE_SIZE = 128
ACTION_SIZE = 10
MODEL_PATH = "C:/Users/deads/OneDrive/Documents/AGI/DEADLOCK-TRANSISTOR-DIO/DEADLOCK-TRANSISTOR-DIO-SRC/Q_Layered_Network/reasoning_dqn_model.pth"

# Create a global agent instance to avoid reloading the model on every call
agent = None

def initialize_agent():
    """Initializes the DQNAgent and loads the pre-trained model."""
    global agent
    if agent is None:
        print("Initializing Python agent and loading model...")
        # Check if the model file exists
        if not os.path.exists(MODEL_PATH):
            raise FileNotFoundError(f"Model file not found at {MODEL_PATH}")
        
        # Instantiate the DQNAgent
        agent = DQNAgent(STATE_SIZE, ACTION_SIZE)
        
        # Load the trained model
        try:
            agent.load(MODEL_PATH)
            agent.q_network.eval()  # Set the network to evaluation mode
            print("Python agent initialized successfully.")
        except Exception as e:
            raise RuntimeError(f"Failed to load the model: {e}")

def get_action(state_list: list) -> int:
    """
    Takes a state (as a list of floats) and returns the predicted action index.
    """
    try:
        # Ensure the agent is initialized
        if agent is None:
            initialize_agent()

        # Convert the input list to a PyTorch tensor of the correct shape
        state_tensor = torch.tensor(state_list, dtype=torch.float32).view(1, -1)

        # Check if the input tensor shape is correct
        if state_tensor.shape != (1, STATE_SIZE):
            return -1 # Indicate an error

        # Get Q-values from the model
        with torch.no_grad():
            q_values = agent.q_network(state_tensor)
        
        # Get the action with the highest Q-value
        action_index = torch.argmax(q_values).item()
        
        return action_index

    except Exception as e:
        print(f"An error occurred in get_action: {e}")
        return -1 # Indicate an error

# Initialize the agent when the module is first imported
initialize_agent()

# Example of how to use it (for testing this script directly)
if __name__ == "__main__":
    print("Testing rust_interface.py...")
    # Create a dummy state
    dummy_state = [0.1] * STATE_SIZE
    action = get_action(dummy_state)
    print(f"Test state: A list of {len(dummy_state)} floats")
    print(f"Predicted action: {action}")
    # The action should be a number between 0 and 9. 
    # If it's -1, an error occurred.
    assert 0 <= action < ACTION_SIZE
    print("Test successful!")
