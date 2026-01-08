## Array Manipulation Functions

Array manipulation: sorting, grouping, chunking, filtering, and transformation.

**See also:** [Function Reference](../FUNCTIONS.md) | [Main Documentation](../README.md)

Utility functions for working with arrays.

#### `array_count(array)`

Count the number of items in an array (alias for length).

**Arguments:**
- `array` (required): Array to count

**Returns:** Number of items in the array

**Example:**
```jinja
{# Count items #}
{% set items = ["apple", "banana", "cherry"] %}
Total: {{ array_count(array=items) }}
{# Output: Total: 3 #}

{# Empty array #}
{% set empty = [] %}
Count: {{ array_count(array=empty) }}
{# Output: Count: 0 #}

{# Conditional based on count #}
{% set tasks = ["task1", "task2", "task3"] %}
{% if array_count(array=tasks) > 2 %}
Multiple tasks pending
{% endif %}
```

#### `array_chunk(array, size)`

Split an array into chunks of specified size.

**Arguments:**
- `array` (required): Array to split
- `size` (required): Size of each chunk (must be > 0)

**Returns:** Array of arrays, where each sub-array has at most `size` elements

**Example:**
```jinja
{# Split into pairs #}
{% set nums = [1, 2, 3, 4, 5, 6] %}
{% for chunk in array_chunk(array=nums, size=2) %}
  Chunk: {{ chunk }}
{% endfor %}
{# Output:
   Chunk: [1, 2]
   Chunk: [3, 4]
   Chunk: [5, 6]
#}

{# Pagination #}
{% set items = ["a", "b", "c", "d", "e", "f", "g"] %}
{% for page in array_chunk(array=items, size=3) %}
  Page {{ loop.index }}: {{ page | join(", ") }}
{% endfor %}
{# Output:
   Page 1: a, b, c
   Page 2: d, e, f
   Page 3: g
#}

{# Grid layout #}
{% set products = ["Product1", "Product2", "Product3", "Product4"] %}
{% for row in array_chunk(array=products, size=2) %}
<div class="row">
  {% for item in row %}
  <div class="col">{{ item }}</div>
  {% endfor %}
</div>
{% endfor %}
```

#### `array_zip(array1, array2)`

Combine two arrays into pairs (like a zipper).

**Arguments:**
- `array1` (required): First array
- `array2` (required): Second array

**Returns:** Array of two-element arrays (pairs). Length is the minimum of the two input arrays.

**Example:**
```jinja
{# Combine keys and values #}
{% set keys = ["name", "age", "city"] %}
{% set values = ["Alice", 30, "NYC"] %}
{% for pair in array_zip(array1=keys, array2=values) %}
  {{ pair[0] }}: {{ pair[1] }}
{% endfor %}
{# Output:
   name: Alice
   age: 30
   city: NYC
#}

{# Configuration mapping #}
{% set env_vars = ["HOST", "PORT", "DEBUG"] %}
{% set defaults = ["localhost", "8080", "false"] %}
{% for pair in array_zip(array1=env_vars, array2=defaults) %}
{{ pair[0] }}={{ pair[1] }}
{% endfor %}

{# Different lengths - stops at shorter #}
{% set a = [1, 2, 3, 4] %}
{% set b = ["a", "b"] %}
{{ array_zip(array1=a, array2=b) }}
{# Output: [[1, "a"], [2, "b"]] #}
```

**Real-world use case - Environment variables with defaults:**
```jinja
{% set var_names = ["DATABASE_HOST", "DATABASE_PORT", "DATABASE_NAME", "DATABASE_USER"] %}
{% set defaults = ["localhost", "5432", "myapp", "postgres"] %}

# Database configuration
{% for pair in array_zip(array1=var_names, array2=defaults) %}
export {{ pair[0] }}="${{ pair[0] }}:-{{ pair[1] }}}"
{% endfor %}

{# Output:
export DATABASE_HOST="${DATABASE_HOST:-localhost}"
export DATABASE_PORT="${DATABASE_PORT:-5432}"
export DATABASE_NAME="${DATABASE_NAME:-myapp}"
export DATABASE_USER="${DATABASE_USER:-postgres}"
#}
```

#### `array_sort_by(array, key)`

Sort an array of objects by a specified key.

**Arguments:**
- `array` (required): Array of objects to sort
- `key` (required): Object key name to sort by

**Returns:** New array sorted by the key value (ascending order)

**Example:**
```jinja
{# Sort users by age #}
{% set users = [
  {"name": "Alice", "age": 30},
  {"name": "Bob", "age": 25},
  {"name": "Charlie", "age": 35}
] %}
{% for user in array_sort_by(array=users, key="age") %}
  {{ user.name }}: {{ user.age }}
{% endfor %}
{# Output:
   Bob: 25
   Alice: 30
   Charlie: 35
#}

{# Sort by string key #}
{% set products = [
  {"name": "Zebra Toy", "price": 15},
  {"name": "Apple Pie", "price": 10},
  {"name": "Mango Juice", "price": 12}
] %}
{% for product in array_sort_by(array=products, key="name") %}
  {{ product.name }}
{% endfor %}
{# Output: Apple Pie, Mango Juice, Zebra Toy #}
```

#### `array_group_by(array, key)`

Group array items by a key value.

**Arguments:**
- `array` (required): Array of objects to group
- `key` (required): Object key name to group by

**Returns:** Object with keys as group names and values as arrays of grouped items

**Example:**
```jinja
{# Group users by department #}
{% set users = [
  {"name": "Alice", "dept": "Engineering"},
  {"name": "Bob", "dept": "Sales"},
  {"name": "Charlie", "dept": "Engineering"}
] %}
{% set grouped = array_group_by(array=users, key="dept") %}
{% for dept, members in grouped | items %}
  {{ dept }}:
  {% for user in members %}
    - {{ user.name }}
  {% endfor %}
{% endfor %}
{# Output:
   Engineering:
     - Alice
     - Charlie
   Sales:
     - Bob
#}

{# Group by numeric value #}
{% set tasks = [
  {"name": "Task1", "priority": 1},
  {"name": "Task2", "priority": 2},
  {"name": "Task3", "priority": 1}
] %}
{% set by_priority = array_group_by(array=tasks, key="priority") %}
High priority: {{ by_priority["1"] | length }} tasks
```

#### `array_unique(array)` / `| array_unique`

Remove duplicate values from an array.

**Arguments:**
- `array` (required): Array to deduplicate

**Returns:** New array with duplicates removed (first occurrence kept)

**Function syntax:**
```jinja
{# Remove duplicate numbers #}
{% set nums = [1, 2, 2, 3, 1, 4, 3, 5] %}
{{ array_unique(array=nums) }}
{# Output: [1, 2, 3, 4, 5] #}

{# Unique tags #}
{% set tags = ["docker", "kubernetes", "docker", "helm", "kubernetes"] %}
Unique tags: {{ array_unique(array=tags) | join(", ") }}
{# Output: Unique tags: docker, kubernetes, helm #}
```

**Filter syntax:**
```jinja
{% set nums = [1, 2, 2, 3, 3, 3] %}
{{ nums | array_unique | join(", ") }}
{# Output: 1, 2, 3 #}

{# Chaining with sum #}
{{ nums | array_unique | array_sum }}
{# Output: 6 #}
```

#### `array_flatten(array)` / `| array_flatten`

Flatten nested arrays by one level.

**Arguments:**
- `array` (required): Array with nested arrays

**Returns:** New array with nested arrays flattened one level

**Function syntax:**
```jinja
{# Flatten nested arrays #}
{% set nested = [[1, 2], [3, 4], [5]] %}
{{ array_flatten(array=nested) }}
{# Output: [1, 2, 3, 4, 5] #}

{# Only flattens one level #}
{% set deep = [[1, [2, 3]], [4]] %}
{{ array_flatten(array=deep) }}
{# Output: [1, [2, 3], 4] #}
```

**Filter syntax:**
```jinja
{% set nested = [[1, 2], [3, 4], [5]] %}
{{ nested | array_flatten | join(", ") }}
{# Output: 1, 2, 3, 4, 5 #}

{# Collect values from multiple sources #}
{% set server1_ips = ["10.0.1.1", "10.0.1.2"] %}
{% set server2_ips = ["10.0.2.1"] %}
{{ [server1_ips, server2_ips] | array_flatten | join(", ") }}
{# Output: 10.0.1.1, 10.0.1.2, 10.0.2.1 #}
```

**Real-world use case - Task management dashboard:**
```jinja
{% set tasks = [
  {"name": "Fix bug #123", "status": "done", "assignee": "Alice"},
  {"name": "Deploy v2.0", "status": "in_progress", "assignee": "Bob"},
  {"name": "Write docs", "status": "done", "assignee": "Alice"},
  {"name": "Code review", "status": "pending", "assignee": "Charlie"}
] %}

{# Group by status #}
{% set by_status = array_group_by(array=tasks, key="status") %}

Task Status Dashboard:
{% for status, items in by_status | items %}
{{ status | upper }} ({{ items | length }} tasks):
  {% for task in array_sort_by(array=items, key="name") %}
  - {{ task.name }} ({{ task.assignee }})
  {% endfor %}
{% endfor %}

{# Get unique assignees #}
{% set all_assignees = [] %}
{% for task in tasks %}
  {% set _ = all_assignees.append(task.assignee) %}
{% endfor %}
Unique assignees: {{ array_unique(array=all_assignees) | join(", ") }}
```

#### `array_take(array, n)`

Take the first N elements from an array.

**Arguments:**
- `array` (required): Source array
- `n` (required): Number of elements to take

**Returns:** Array with the first N elements

**Example:**
```jinja
{{ array_take(array=[1, 2, 3, 4, 5], n=3) }}
{# Output: [1, 2, 3] #}

{# Taking more than available returns all elements #}
{{ array_take(array=[1, 2], n=5) }}
{# Output: [1, 2] #}
```

#### `array_drop(array, n)`

Skip the first N elements from an array.

**Arguments:**
- `array` (required): Source array
- `n` (required): Number of elements to skip

**Returns:** Array with elements after the first N

**Example:**
```jinja
{{ array_drop(array=[1, 2, 3, 4, 5], n=2) }}
{# Output: [3, 4, 5] #}

{# Dropping more than available returns empty array #}
{{ array_drop(array=[1, 2], n=5) }}
{# Output: [] #}
```

#### `array_index_of(array, value)`

Find the index of an element in an array.

**Arguments:**
- `array` (required): Array to search
- `value` (required): Value to find

**Returns:** Index (0-based) or -1 if not found

**Example:**
```jinja
{{ array_index_of(array=["a", "b", "c"], value="b") }}
{# Output: 1 #}

{{ array_index_of(array=[1, 2, 3], value=5) }}
{# Output: -1 #}
```

#### `array_find(array, key, value)`

Find the first matching object in an array of objects.

**Arguments:**
- `array` (required): Array of objects to search
- `key` (required): Key to match
- `value` (required): Value to match

**Returns:** The first matching object or null if not found

**Example:**
```jinja
{% set users = [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}] %}
{{ array_find(array=users, key="id", value=2) | tojson }}
{# Output: {"id": 2, "name": "Bob"} #}

{{ array_find(array=users, key="id", value=99) }}
{# Output: null #}
```

#### `array_filter_by(array, key, op, value)`

Filter an array of objects by a key with comparison operators.

**Arguments:**
- `array` (required): Array of objects to filter
- `key` (required): Key to compare
- `op` (required): Operator: `"eq"`, `"ne"`, `"gt"`, `"lt"`, `"gte"`, `"lte"`, `"contains"`
- `value` (required): Value to compare against

**Returns:** Filtered array of matching objects

**Example:**
```jinja
{% set items = [{"price": 10}, {"price": 20}, {"price": 30}] %}
{{ array_filter_by(array=items, key="price", op="gt", value=15) | tojson }}
{# Output: [{"price": 20}, {"price": 30}] #}

{% set users = [{"name": "Alice"}, {"name": "Bob"}, {"name": "Charlie"}] %}
{{ array_filter_by(array=users, key="name", op="contains", value="li") | tojson }}
{# Output: [{"name": "Alice"}, {"name": "Charlie"}] #}
```

#### `array_pluck(array, key)`

Extract values from an array of objects by key (supports dot notation for nested keys).

**Arguments:**
- `array` (required): Array of objects
- `key` (required): Key path to extract (e.g., `"user.name"`)

**Returns:** Array of extracted values

**Example:**
```jinja
{% set users = [{"name": "Alice"}, {"name": "Bob"}] %}
{{ array_pluck(array=users, key="name") | tojson }}
{# Output: ["Alice", "Bob"] #}

{# Nested keys with dot notation #}
{% set data = [{"user": {"name": "Alice"}}, {"user": {"name": "Bob"}}] %}
{{ array_pluck(array=data, key="user.name") | tojson }}
{# Output: ["Alice", "Bob"] #}
```

#### `array_intersection(array1, array2)`

Get elements that exist in both arrays.

**Arguments:**
- `array1` (required): First array
- `array2` (required): Second array

**Returns:** Array of common elements

**Example:**
```jinja
{{ array_intersection(array1=[1, 2, 3, 4], array2=[3, 4, 5, 6]) | tojson }}
{# Output: [3, 4] #}

{{ array_intersection(array1=["a", "b", "c"], array2=["b", "c", "d"]) | tojson }}
{# Output: ["b", "c"] #}
```

#### `array_difference(array1, array2)`

Get elements in the first array that are not in the second.

**Arguments:**
- `array1` (required): First array
- `array2` (required): Second array

**Returns:** Array of elements in array1 but not in array2

**Example:**
```jinja
{{ array_difference(array1=[1, 2, 3, 4], array2=[3, 4, 5, 6]) | tojson }}
{# Output: [1, 2] #}

{{ array_difference(array1=["a", "b", "c"], array2=["b"]) | tojson }}
{# Output: ["a", "c"] #}
```

#### `array_union(array1, array2)`

Get all unique elements from both arrays.

**Arguments:**
- `array1` (required): First array
- `array2` (required): Second array

**Returns:** Array of all unique elements from both arrays

**Example:**
```jinja
{{ array_union(array1=[1, 2, 3], array2=[3, 4, 5]) | tojson }}
{# Output: [1, 2, 3, 4, 5] #}

{{ array_union(array1=["a", "b"], array2=["b", "c"]) | tojson }}
{# Output: ["a", "b", "c"] #}
```

#### `array_symmetric_difference(array1, array2)`

Get elements that are in either array but not in both.

**Arguments:**
- `array1` (required): First array
- `array2` (required): Second array

**Returns:** Array of elements in either array but not in both

**Example:**
```jinja
{{ array_symmetric_difference(array1=[1, 2, 3, 4], array2=[3, 4, 5, 6]) | tojson }}
{# Output: [1, 2, 5, 6] #}

{{ array_symmetric_difference(array1=["a", "b", "c"], array2=["b", "c", "d"]) | tojson }}
{# Output: ["a", "d"] #}
```

