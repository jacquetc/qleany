// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "repository/interface_passenger_repository.h"

#include "result.h"

using namespace FrontEnds;

using namespace FrontEnds::Contracts::Repository;

namespace FrontEnds::Contracts::CQRS::Passenger::Validators
{
class RemovePassengerCommandValidator
{
public:
    RemovePassengerCommandValidator(InterfacePassengerRepository *passengerRepository)
        : m_passengerRepository(passengerRepository)
    {
    }

    Result<void> validate(int id) const

    {
        Result<bool> existsResult = m_passengerRepository->exists(id);

        if (!existsResult.value()) {
            return Result<void>(QLN_ERROR_1(Q_FUNC_INFO, Error::Critical, "id_not_found"));
        }

        // Return that is Ok :
        return Result<void>();
    }

private:
    InterfacePassengerRepository *m_passengerRepository;
};
} // namespace FrontEnds::Contracts::CQRS::Passenger::Validators