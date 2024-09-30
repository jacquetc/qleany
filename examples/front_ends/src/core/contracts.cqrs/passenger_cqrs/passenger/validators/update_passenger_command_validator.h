// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "passenger/update_passenger_dto.h"

#include "repository/interface_passenger_repository.h"

#include "result.h"

using namespace FrontEnds;

using namespace FrontEnds::Contracts::Repository;

using namespace FrontEnds::Contracts::DTO::Passenger;

namespace FrontEnds::Contracts::CQRS::Passenger::Validators
{
class UpdatePassengerCommandValidator
{
public:
    UpdatePassengerCommandValidator(InterfacePassengerRepository *passengerRepository)
        : m_passengerRepository(passengerRepository)
    {
    }

    Result<void> validate(const UpdatePassengerDTO &dto) const

    {
        Result<bool> existsResult = m_passengerRepository->exists(dto.id());

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