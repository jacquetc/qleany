// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "passenger/create_passenger_dto.h"

#include "repository/interface_passenger_repository.h"

#include <qleany/common/result.h>

using namespace Qleany;

using namespace Simple::Contracts::Repository;

using namespace Simple::Contracts::DTO::Passenger;

namespace Simple::Contracts::CQRS::Passenger::Validators
{
class CreatePassengerCommandValidator
{
  public:
    CreatePassengerCommandValidator(InterfacePassengerRepository *passengerRepository)
        : m_passengerRepository(passengerRepository)
    {
    }

    Result<void> validate(const CreatePassengerDTO &dto) const

    {

        // Return that is Ok :
        return Result<void>();
    }

  private:
    InterfacePassengerRepository *m_passengerRepository;
};
} // namespace Simple::Contracts::CQRS::Passenger::Validators