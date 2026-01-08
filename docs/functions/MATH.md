## Math Functions

Mathematical functions: min, max, abs, round, ceil, floor, and percentage calculations.

**See also:** [Function Reference](../FUNCTIONS.md) | [Main Documentation](../README.md)

Perform mathematical calculations and operations.

#### `min(a, b)`

Return the minimum of two values.

**Arguments:**
- `a` (required): First number
- `b` (required): Second number

**Returns:** The smaller of the two values

**Example:**
```jinja
{# Find minimum #}
{{ min(a=10, b=20) }}
{# Output: 10 #}

{# With variables #}
{% set cpu1 = 45.2 %}
{% set cpu2 = 38.7 %}
Lowest CPU: {{ min(a=cpu1, b=cpu2) }}%
```

#### `max(a, b)`

Return the maximum of two values.

**Arguments:**
- `a` (required): First number
- `b` (required): Second number

**Returns:** The larger of the two values

**Example:**
```jinja
{# Find maximum #}
{{ max(a=10, b=20) }}
{# Output: 20 #}

{# With variables #}
{% set memory1 = 2048 %}
{% set memory2 = 4096 %}
Peak memory: {{ max(a=memory1, b=memory2) }}MB
```

#### `abs(number)`

Return the absolute value of a number. Supports both function and filter syntax.

**Arguments:**
- `number` (required): Number to get absolute value of

**Returns:** The absolute value (always positive)

**Example:**
```jinja
{# Function syntax #}
{{ abs(number=-42) }}
{# Output: 42 #}

{# Filter syntax #}
{{ -42 | abs }}
{# Output: 42 #}

{# Temperature difference - function syntax #}
{% set temp1 = 25 %}
{% set temp2 = 18 %}
Difference: {{ abs(number=temp1 - temp2) }}°C

{# Chaining filters #}
{{ -3.7 | abs | ceil }}
{# Output: 4 #}
```

#### `round(number, decimals=0)`

Round a number to N decimal places. Supports both function and filter syntax.

**Arguments:**
- `number` (required): Number to round
- `decimals` (optional): Number of decimal places (default: 0)

**Returns:** The number rounded to the specified decimal places

**Example:**
```jinja
{# Function syntax #}
{{ round(number=3.7) }}
{# Output: 4 #}

{# Filter syntax #}
{{ 3.7 | round }}
{# Output: 4 #}

{# Round to 2 decimal places - function syntax #}
{{ round(number=3.14159, decimals=2) }}
{# Output: 3.14 #}

{# Round to 2 decimal places - filter syntax #}
{{ 3.14159 | round(decimals=2) }}
{# Output: 3.14 #}

{# Price calculation with filter chaining #}
{% set price = 19.999 %}
Price: ${{ price | round(decimals=2) }}
```

#### `ceil(number)`

Round up to the nearest integer. Supports both function and filter syntax.

**Arguments:**
- `number` (required): Number to round up

**Returns:** The smallest integer greater than or equal to the number

**Example:**
```jinja
{# Function syntax #}
{{ ceil(number=3.1) }}
{# Output: 4 #}

{# Filter syntax #}
{{ 3.1 | ceil }}
{# Output: 4 #}

{# Calculate required servers - function syntax #}
{% set users = 150 %}
{% set users_per_server = 50 %}
Servers needed: {{ ceil(number=users / users_per_server) }}

{# With filter chaining #}
{{ -3.7 | abs | ceil }}
{# Output: 4 #}
```

#### `floor(number)`

Round down to the nearest integer. Supports both function and filter syntax.

**Arguments:**
- `number` (required): Number to round down

**Returns:** The largest integer less than or equal to the number

**Example:**
```jinja
{# Function syntax #}
{{ floor(number=3.9) }}
{# Output: 3 #}

{# Filter syntax #}
{{ 3.9 | floor }}
{# Output: 3 #}

{# Calculate filled pages - function syntax #}
{% set items = 47 %}
{% set items_per_page = 10 %}
Full pages: {{ floor(number=items / items_per_page) }}

{# With filter chaining #}
{{ 3.999 | floor | abs }}
{# Output: 3 #}
```

#### `percentage(value, total)`

Calculate percentage.

**Arguments:**
- `value` (required): The part value
- `total` (required): The total/whole value

**Returns:** The percentage (0-100)

**Example:**
```jinja
{# Calculate percentage #}
{{ percentage(value=25, total=100) }}
{# Output: 25.0 #}

{# Progress calculation #}
{% set completed = 7 %}
{% set total_tasks = 10 %}
Progress: {{ round(number=percentage(value=completed, total=total_tasks), decimals=1) }}%

{# Disk usage #}
{% set used = 450 %}
{% set capacity = 500 %}
Disk usage: {{ round(number=percentage(value=used, total=capacity), decimals=2) }}%
```

