## Statistical Functions

Statistical functions: sum, average, median, min, and max for arrays.

**See also:** [Function Reference](../FUNCTIONS.md) | [Main Documentation](../README.md)

Calculate statistics on numeric arrays.

#### `array_sum(array)` / `| array_sum`

Calculate the sum of all values in an array.

**Arguments:**
- `array` (required): Array of numbers to sum

**Returns:** Sum of all values (integer if no decimals, float otherwise)

**Function syntax:**
```jinja
{# Sum of integers #}
{% set numbers = [1, 2, 3, 4, 5] %}
Total: {{ array_sum(array=numbers) }}
{# Output: Total: 15 #}

{# Sum of prices #}
{% set prices = [10.5, 20.25, 5.75] %}
Total: ${{ array_sum(array=prices) }}
{# Output: Total: $36.5 #}
```

**Filter syntax:**
```jinja
{% set numbers = [1, 2, 3, 4, 5] %}
{{ numbers | array_sum }}
{# Output: 15 #}

{# Chaining: unique then sum #}
{% set nums = [1, 2, 2, 3, 3] %}
{{ nums | array_unique | array_sum }}
{# Output: 6 #}
```

#### `array_avg(array)` / `| array_avg`

Calculate the average (mean) of all values in an array.

**Arguments:**
- `array` (required): Array of numbers

**Returns:** Arithmetic mean of all values (0 for empty arrays)

**Function syntax:**
```jinja
{# Average score #}
{% set scores = [85, 90, 78, 92, 88] %}
Average: {{ array_avg(array=scores) }}
{# Output: Average: 86.6 #}

{# Empty array handling #}
{% set empty = [] %}
Default: {{ array_avg(array=empty) }}
{# Output: Default: 0 #}
```

**Filter syntax:**
```jinja
{% set scores = [85, 90, 78, 92, 88] %}
{{ scores | array_avg }}
{# Output: 86.6 #}
```

#### `array_median(array)` / `| array_median`

Calculate the median value of an array.

**Arguments:**
- `array` (required): Array of numbers

**Returns:** Middle value for odd-length arrays, average of two middle values for even-length arrays

**Function syntax:**
```jinja
{# Median of odd-length array #}
{% set nums = [1, 3, 5, 7, 9] %}
Median: {{ array_median(array=nums) }}
{# Output: Median: 5 #}

{# Median of even-length array #}
{% set nums = [1, 2, 3, 4] %}
Median: {{ array_median(array=nums) }}
{# Output: Median: 2.5 #}
```

**Filter syntax:**
```jinja
{% set nums = [1, 3, 5, 7, 9] %}
{{ nums | array_median }}
{# Output: 5 #}
```

#### `array_min(array)` / `| array_min`

Find the minimum value in an array.

**Arguments:**
- `array` (required): Array of numbers

**Returns:** Smallest value in the array

**Function syntax:**
```jinja
{# Find minimum #}
{% set numbers = [42, 17, 99, 8, 55] %}
Minimum: {{ array_min(array=numbers) }}
{# Output: Minimum: 8 #}

{# Lowest price #}
{% set prices = [10.99, 5.49, 15.99, 7.25] %}
Best deal: ${{ array_min(array=prices) }}
```

**Filter syntax:**
```jinja
{% set numbers = [42, 17, 99, 8, 55] %}
{{ numbers | array_min }}
{# Output: 8 #}
```

#### `array_max(array)` / `| array_max`

Find the maximum value in an array.

**Arguments:**
- `array` (required): Array of numbers

**Returns:** Largest value in the array

**Function syntax:**
```jinja
{# Find maximum #}
{% set numbers = [42, 17, 99, 8, 55] %}
Maximum: {{ array_max(array=numbers) }}
{# Output: Maximum: 99 #}

{# Peak memory usage #}
{% set memory = [512, 768, 1024, 896] %}
Peak: {{ array_max(array=memory) }}MB
```

**Filter syntax:**
```jinja
{% set numbers = [42, 17, 99, 8, 55] %}
{{ numbers | array_max }}
{# Output: 99 #}
```

**Real-world use case - Resource allocation:**
```jinja
{% set cpu_usage = [45, 62, 78, 55, 91, 67] %}
{% set mem_usage = [2048, 3072, 4096, 2560] %}

CPU Statistics:
  Average: {{ array_avg(array=cpu_usage) }}%
  Peak: {{ array_max(array=cpu_usage) }}%
  Median: {{ array_median(array=cpu_usage) }}%

Memory Statistics:
  Total: {{ array_sum(array=mem_usage) }}MB
  Average: {{ array_avg(array=mem_usage) }}MB
  Peak: {{ array_max(array=mem_usage) }}MB

{% if array_max(array=cpu_usage) > 90 %}
Alert: High CPU usage detected!
{% endif %}
```

