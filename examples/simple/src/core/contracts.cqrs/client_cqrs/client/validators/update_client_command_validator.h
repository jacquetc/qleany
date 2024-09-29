// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "client/update_client_dto.h"

#include "repository/interface_client_repository.h"

#include "result.h"

using namespace;

using namespace Simple::Contracts::Repository;

using namespace Simple::Contracts::DTO::Client;

namespace Simple::Contracts::CQRS::Client::Validators
{
class UpdateClientCommandValidator
{
  public:
    UpdateClientCommandValidator(InterfaceClientRepository *clientRepository) : m_clientRepository(clientRepository)
    {
    }

    Result<void> validate(const UpdateClientDTO &dto) const

    {

        Result<bool> existsResult = m_clientRepository->exists(dto.id());

        if (!existsResult.value())
        {
            return Result<void>(QLN_ERROR_1(Q_FUNC_INFO, Error::Critical, "id_not_found"));
        }

        // Return that is Ok :
        return Result<void>();
    }

  private:
    InterfaceClientRepository *m_clientRepository;
};
} // namespace Simple::Contracts::CQRS::Client::Validators