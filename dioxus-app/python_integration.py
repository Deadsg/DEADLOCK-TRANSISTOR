# python_integration.py
import random
import numpy as np

# Global agent reference (if using a loaded model)
dqn_agent = None

def init_dqn():
    global dqn_agent
    from your_dqn_module import DQNAgent  # or however your DQN is defined
    dqn_agent = DQNAgent()
    dqn_agent.load_model("C:\Users\deads\OneDrive\Documents\AGI\DEADLOCK-TRANSISTOR\Q_Layered_Network\dqn_node_model.onnx")
    return "DQN initialized"

def get_dqn_action(state):
    """Returns the next action for a given state."""
    global dqn_agent
    if dqn_agent is None:
        raise RuntimeError("DQN agent not initialized. Call init_dqn() first.")
    state_array = np.array(state, dtype=np.float32)
    action = dqn_agent.select_action(state_array)
    return action

_dqn_initialized = False

def init_dqn():
    global _dqn_initialized
    if not _dqn_initialized:
        print("🔧 Initializing mock Deep Q Network...")
        _dqn_initialized = True
    return "initialized"

def get_dqn_action(state):
    if not _dqn_initialized:
        raise RuntimeError("DQN not initialized.")
    print(f"Received state: {state}")
    return float(sum(state)) / len(state) if state else 0.0
