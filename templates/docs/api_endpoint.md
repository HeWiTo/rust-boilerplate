# {{endpoint_name}}

## Description

{{endpoint_description}}

## Endpoint

`{{http_method}} {{endpoint_url}}`

## Parameters

| Name | Type | Description | Required |
|------|------|-------------|----------|
{{#each parameters}}
| {{this.name}} | {{this.type}} | {{this.description}} | {{this.required}} |
{{/each}}

## Request Example

```json
{{request_example}}
```

## Response

### Success Response

**Code:** {{success_code}}

**Content Example:**

```json
{{success_response_example}}
```

### Error Response

**Condition:** {{error_condition}}

**Code:** {{error_code}}

**Content Example:**

```json
{{error_response_example}}
```

## Notes

{{additional_notes}}