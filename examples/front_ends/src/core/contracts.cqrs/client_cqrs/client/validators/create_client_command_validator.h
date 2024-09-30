// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "client/create_client_dto.h"

#include "repository/interface_client_repository.h"

#include "result.h"

using namespace FrontEnds;

using namespace FrontEnds::Contracts::Repository;

using namespace FrontEnds::Contracts::DTO::Client;

namespace FrontEnds::Contracts::CQRS::Client::Validators
{
class CreateClientCommandValidator
{
public:
    CreateClientCommandValidator(InterfaceClientRepository *clientRepository)
        : m_clientRepository(clientRepository)
    {
    }

    Result<void> validate(const CreateClientDTO &dto) const

    {
        // Return that is Ok :
        return Result<void>();
    }

private:
    InterfaceClientRepository *m_clientRepository;
};
} // namespace FrontEnds::Contracts::CQRS::Client::Validators