# handles the entities
add_subdirectory(entities)
# stores the interfaces
add_subdirectory(contracts)
# handles the data access from inside the system
add_subdirectory(persistence)
# handles the data access from outside the system
add_subdirectory(contracts.dto)
# handles the CQRS commands, queries and validators
add_subdirectory(contracts.cqrs)
# handles the use cases
add_subdirectory(application)
# handles the interaction between the system's inner layers (use cases, DTOs) and the external world 
add_subdirectory(interactor) 
