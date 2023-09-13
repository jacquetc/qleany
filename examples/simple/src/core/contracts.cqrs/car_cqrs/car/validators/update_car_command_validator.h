// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "car/update_car_dto.h"


#include "repository/interface_car_repository.h"

#include "qleany/common/result.h"

using namespace Qleany;

using namespace Simple::Contracts::Repository;

using namespace Simple::Contracts::DTO::Car;

namespace Simple::Contracts::CQRS::Car::Validators
{
class UpdateCarCommandValidator
{
  public:
    UpdateCarCommandValidator(InterfaceCarRepository *carRepository)
        :  m_carRepository(carRepository)
    {
    }

    Result<void> validate(const UpdateCarDTO &dto) const

    {




        Result<bool> existsResult = m_carRepository->exists(dto.id());

        if (!existsResult.value())
        {
            return Result<void>(QLN_ERROR_1(Q_FUNC_INFO, Error::Critical, "id_already_exists"));
        }




        // Return that is Ok :
        return Result<void>();
    }

  private:

    InterfaceCarRepository *m_carRepository;

};
} // namespace Simple::Contracts::CQRS::Car::Validators