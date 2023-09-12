// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "car/create_car_dto.h"


#include "repository/interface_car_repository.h"

#include "qleany/common/result.h"

using namespace Qleany;

using namespace Simple::Contracts::Repository;

using namespace Simple::Contracts::DTO::Car;

namespace Simple::Contracts::CQRS::Car::Validators
{
class CreateCarCommandValidator
{
  public:
    CreateCarCommandValidator(InterfaceCarRepository *carRepository)
        :  m_carRepository(carRepository)
    {
    }

    Result<void> validate(const CreateCarDTO &dto) const

    {




        // Return that is Ok :
        return Result<void>();
    }

  private:

    InterfaceCarRepository *m_carRepository;

};
} // namespace Simple::Contracts::CQRS::Car::Validators