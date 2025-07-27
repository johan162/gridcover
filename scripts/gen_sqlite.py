import json
import sys

def generate_sqlite_statements(file_path):
    """
    Reads a JSON file and generates SQLite CREATE TABLE and INSERT statements.

    Args:
        file_path (str): The path to the input JSON file.

    Returns:
        tuple: A tuple containing the CREATE TABLE string and the INSERT INTO string.
    """
    with open(file_path, 'r') as f:
        data = json.load(f)

    if not isinstance(data, dict) or not data:
        raise ValueError("JSON root must be a non-empty object.")

    # Get the first key in the JSON object to use as the base for the table name
    root_key = list(data.keys())[0]
    table_name = root_key.lower() + 's'
    data_array_name = root_key.lower()
    json_data = data[root_key]

    columns = []
    insert_columns = []
    insert_params = []

    def get_type_and_function(value, key):
        """Maps Python types to SQLite types and conversion functions."""
        if isinstance(value, float):
            # Print the value for debugging purposes
            # print(f"Processing float value: {value} from key: {key}")
            return "REAL", 'get_f64_from_json(&{name}_data{key})'
        elif isinstance(value, bool):
            # Booleans are stored as integers (0 or 1) in SQLite
            # print(f"Processing boolean value: {value} from key: {key}")
            return "INTEGER", 'get_bool_as_i64_from_json(&{name}_data{key})'
        elif isinstance(value, int):
            # print(f"Processing integer value: {value} from key: {key}")
            return "INTEGER", 'get_i64_from_json(&{name}_data{key})'
        else:
            # print(f"Processing string value: {value} from key: {key}")
            return "TEXT", 'get_string_from_json(&{name}_data{key})'

    def process_json(json_obj, prefix="", key_path=""):
        """Recursively processes the JSON object to flatten keys and values."""
        for key, value in json_obj.items():
            # Sanitize key for SQL column name
            sanitized_key = key.lower().replace(' ', '_').replace('(', '').replace('#', 'num').replace(')', '').replace('%', 'percent').replace('.', '_').replace('/', '_per_')
            new_prefix = f"{prefix}_{sanitized_key}" if prefix else sanitized_key
            
            # Build the key path for the Rust-like access string
            new_key_path = f'{key_path}["{key}"]'

            if isinstance(value, dict):
                process_json(value, new_prefix, new_key_path)
            else:
                sql_type, function_call = get_type_and_function(value, new_key_path)
                
                # Column definition for CREATE TABLE
                columns.append(f"{new_prefix} {sql_type}")
                
                # Column name for INSERT INTO
                insert_columns.append(new_prefix)
                
                # Parameter for the `params!` macro
                insert_params.append(function_call.format(key=new_key_path, name=data_array_name))

    # Start the recursive processing
    process_json(json_data, key_path="")

    # Create the "CREATE TABLE" statement
    create_statement = f"self.conn.execute(\n   \"CREATE TABLE IF NOT EXISTS {table_name} (\n"
    create_statement += "    id INTEGER PRIMARY KEY AUTOINCREMENT,\n"
    if table_name == 'results':
        create_statement += "    model_id INTEGER NOT NULL,\n"
    create_statement += ",\n".join([f"    {col}" for col in columns])
    create_statement += ",\n    created_at DATETIME DEFAULT CURRENT_TIMESTAMP"
    if table_name == 'results':
        create_statement += ",\n    FOREIGN KEY (model_id) REFERENCES models (id)\n"
    create_statement += "  )\",\n   []\n)?;"


    # Create the "INSERT INTO" statement
    insert_statement = f"tx.execute(\n    \"INSERT INTO {table_name} (\n"
    if table_name == 'results':
        insert_statement += "    model_id,\n"
    insert_statement += ",\n".join([f"    {col}" for col in insert_columns])
    insert_statement += "\n) VALUES (\n"
    offset = 1
    if table_name == 'results':
        insert_statement += "    ?1, "
        offset += 1
    insert_statement += ", ".join([f"?{i+offset}" for i in range(len(insert_columns))])
    insert_statement += "\n)\",\nparams![\n"
    if table_name == 'results':
        insert_statement += "    model_id,\n"
    insert_statement += ",\n".join([f"    {param}" for param in insert_params])
    insert_statement += "\n])?;"

    return create_statement, insert_statement

if __name__ == '__main__':
    # Check if a command-line argument for the file path is provided
    if len(sys.argv) != 2:
        # sys.argv[0] is the script name itself
        print(f"Usage: python {sys.argv[0]} <path_to_json_file>")
        sys.exit(1) # Exit with a status code indicating an error

    json_file_path = sys.argv[1]

    try:
        # Generate the statements from the provided file
        create_sql, insert_sql = generate_sqlite_statements(json_file_path)

        # Print the output
        print("//==============\n//TABLE CREATION\n//==============\n")
        print(create_sql)
        print("\n//==============\n//INSERTION\n//==============\n")
        print(insert_sql)
        print("\n//==============\n//END OF SQL STATEMENTS\n//==============\n")

    except FileNotFoundError:
        print(f"Error: The file '{json_file_path}' was not found.", file=sys.stderr)
        sys.exit(1)
    except json.JSONDecodeError:
        print(f"Error: The file '{json_file_path}' contains invalid JSON.", file=sys.stderr)
        sys.exit(1)
    except (ValueError, KeyError) as e:
        print(f"Error processing JSON data: {e}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"An unexpected error occurred: {e}", file=sys.stderr)
        sys.exit(1)