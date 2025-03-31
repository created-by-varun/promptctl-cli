# PromptCTL API Documentation

## Environment Configuration

### Server Environment Variables

```bash
CLAUDE_API_KEY=your_claude_api_key  # Required: Your Claude API key for prompt generation
```

## Base URL

```
http://localhost:3000
```

## Endpoints

### Generate Prompt

Generates an improved version of the provided prompt.

**Endpoint:** `/api/generate`  
**Method:** `POST`  
**Content-Type:** `application/json`

#### Request Body

```json
{
  "prompt": "string",
  "refine": false
}
```

| Field  | Type    | Description                                                         |
| ------ | ------- | ------------------------------------------------------------------- |
| prompt | string  | The original prompt to improve                                      |
| refine | boolean | If true, returns refinement questions instead of an improved prompt |

#### Response (refine: false)

```json
{
  "content": "string",
  "model": "string"
}
```

| Field   | Type   | Description                                            |
| ------- | ------ | ------------------------------------------------------ |
| content | string | The improved prompt                                    |
| model   | string | The model used for generation (e.g., "Claude 3 Haiku") |

#### Response (refine: true)

```json
{
  "questions": ["string"]
}
```

| Field     | Type     | Description                            |
| --------- | -------- | -------------------------------------- |
| questions | string[] | List of questions to refine the prompt |

#### Example

```bash
curl -X POST http://localhost:3000/api/generate \
  -H "Content-Type: application/json" \
  -d '{"prompt": "write a blog post about rust", "refine": false}'
```

### Refine Prompt

Generates an improved prompt using the original prompt and answers to refinement questions.

**Endpoint:** `/api/generate/refine`  
**Method:** `POST`  
**Content-Type:** `application/json`

#### Request Body

```json
{
  "prompt": "string",
  "answers": [
    {
      "question": "string",
      "answer": "string"
    }
  ]
}
```

| Field   | Type   | Description                    |
| ------- | ------ | ------------------------------ |
| prompt  | string | The original prompt to improve |
| answers | array  | List of question-answer pairs  |

#### Response

```json
{
  "content": "string",
  "model": "string"
}
```

| Field   | Type   | Description                                            |
| ------- | ------ | ------------------------------------------------------ |
| content | string | The improved prompt                                    |
| model   | string | The model used for generation (e.g., "Claude 3 Haiku") |

#### Example

```bash
curl -X POST http://localhost:3000/api/generate/refine \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": "write a blog post about rust",
    "answers": [
      {
        "question": "What is the target audience?",
        "answer": "Beginner programmers"
      },
      {
        "question": "What aspects should be covered?",
        "answer": "Memory safety and ownership"
      }
    ]
  }'
```

## Error Responses

All endpoints may return the following error responses:

### 400 Bad Request

```json
{
  "error": "Invalid request body"
}
```

### 500 Internal Server Error

```json
{
  "error": "Claude API key not configured"
}
```

or

```json
{
  "error": "Claude API error: <error message>"
}
```
