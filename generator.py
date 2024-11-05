import random
import sympy as sp

def generate_linear_equation(num_variables, num_equations, output_file):
    """
    Generate a system of linear equations with arbitrary variables that can always be solved, and save them to a file in a custom format.
    
    Parameters:
    num_variables (int): The number of variables in the system of equations.
    num_equations (int): The number of equations in the system.
    output_file (str): The path to the output file.
    
    Returns:
    tuple: A tuple containing the following:
        - A list of sympy.Eq objects representing the linear equations
        - A dictionary mapping variable names to their assigned values
    """
    # Generate variable names
    variables = [sp.Symbol(f'x{i}') for i in range(num_variables)]
    
    # Generate coefficients and constants for the equations
    coefficients = [[random.uniform(1, 5) for _ in range(num_variables)] for _ in range(num_equations)]
    constants = [random.uniform(1, 10) for _ in range(num_equations)]
    
    # Create the linear equations
    equations = [sp.Eq(sp.sympify(sum(coeff * var for coeff, var in zip(coeffs, variables))), const)
                 for coeffs, const in zip(coefficients, constants)]
    
    # Solve the system of equations to get the variable values
    solution = sp.solve(equations, variables)
    
    # Save the equations to a file in the desired format
    with open(output_file, 'w') as f:
        for coeffs, const in zip(coefficients, constants):
            f.write('; '.join(f"{coeff:.2f}" for coeff in coeffs) + f" ; {const:.2f} ;\n")
    
    return equations, solution


num_variables = 3
num_equations = 3
output_file = 'linear_equations.txt'

equations, solution = generate_linear_equation(num_variables, num_equations, output_file)

print("Generated equations:")
for eq in equations:
    print(eq)

print("\nSolution:")
for var, value in solution.items():
    print(f"{var} = {value}")

print(f"\nEquations saved to '{output_file}'")