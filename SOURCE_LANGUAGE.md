
### Source Language

| From | To |
| -- | -- |
| _program_ | _decls_|
| _program_ | _block_ |
| _program_ | _stmts_ |
| _block_ | **{** _decls_ _stmts_ **}** |

### Declarations

| From | To |
| -- | -- |
| _decls_ | _decls_ _decl_|
| _decls_ | **ε** |
| _decl_ | _type_ **id** **;** |

### Types

| From | To |
| -- | -- |
| _type_ | _type_ **\[** _num_ **]** |
| _type_ | _basic_ |
| _basic_ | **void** |
| _basic_ | **int** |
| _basic_ | **float** |
| _basic_ | **boolean** |
| _basic_ | **char** |
| _basic_ | **string** |

### Numbers

| From | To |
| -- | -- |
| _num_ | **NUM** _num_ |
| _num_ | **NUM** **ε** |
| _real_ | _num_ **.** _num_ |
| _real_ | _num_ |

### Statements

| From | To |
| -- | -- |
| _stmts_ | _stmts_ _stmt_ |
| _stmt_ | _asgn_ |
| _stmt_ | _block_ |
| _stmt_ | _func_ **;** |
| _stmt_ | _funcdecl_ |
| _stmt_ | **for** **(** _decl_ **;** _boolexpr_ **)** _block_ |
| _stmt_ | **if** **(** _boolexpr_ **)** _block_ |
| _stmt_ | **if** **(** _boolexpr_ **)** _block_ **else** _block_ |
| _stmt_ | **while** **(** _boolexpr_ **)** _block_ |
| _stmt_ | **do** _block_ **while** **(** _boolexpr_ **)** **;** |
| _stmt_ | **break** **;** |
| _stmt_ | **continue** **;** |
| _stmt_ | _asgn_ |
| _asgn_ | **ID =** _expr_ **;** |
| _asgn_ | **ID +=** _expr_ **;** |
| _asgn_ | **ID -=** _expr_ **;** |

### Functions
| From | To |
| -- | -- |
| _func_ |  **ID (** _params_ **)** |
| _params_ |  **ID,** _params_|
| _params_ |  **ID** |
| _params_ | **ε** |
| _funcdecl_ | **func** _type_ **ID (** _paramsdecl_ **)** _block_ |
| _paramsdecl_ | _type_ **ID** **,** _paramsdecl_|
| _paramsdecl_ | _type_ **ID** |
| _paramsdecl_ | **ε** |

### Expressions

| From | To |
| -- | -- |
| _boolexpr_ | _boolexpr_ **\|\|** _join_ |
| _boolexpr_ | _join_ |
| _join_ | _join_ **&&** _equality_ |
| _join_ | _equality_ |
| _equality_ | _equality_ **\=\=** _cmp_ |
| _equality_ | _equality_ **\!\=** _cmp_ |
| _equality_ | _cmp_ |
| _cmp_ | _expr_ **<** _expr_ |
| _cmp_ | _expr_ **<=** _expr_ |
| _cmp_ | _expr_ **>** _expr_ |
| _cmp_ | _expr_ **>=** _expr_ |
| _expr_ | _expr_ **+** _bnr_ |
| _expr_ | _expr_ **-** _bnr_ |
| _expr_ | _bnr_ |
| _bnr_ | _bnr_ **&** _term_ |
| _bnr_ | _bnr_ **\|** _term_ |
| _bnr_ | _term_ |
| _term_ | _term_ **\*** _unit_ |
| _term_ | _term_ **\/** _unit_ |
| _term_ | _term_ **%** _unit_ |
| _term_ | _unit_ |
| _unit_ | **-** unit |
| _unit_ | **++** unit |
| _unit_ | **--** unit |
| _unit_ | _factor_ |
| _factor_ | **(** _expr_ **)**  |
| _factor_ | _num_  |
| _factor_ | _real_  |
| _factor_ | _func_  |

