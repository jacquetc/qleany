// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once



#include "repository/interface_car_repository.h"

#include "qleany/common/result.h"

using namespace Qleany;

using namespace Simple::Contracts::Repository;



namespace Simple::Contracts::CQRS::Car::Validators
{
class RemoveCarCommandValidator
{
  public:
    RemoveCarCommandValidator(InterfaceCarRepository *carRepository)
        :  m_carRepository(carRepository)
    {
    }

    Result<void> validate(int id) const

    {




        Result<bool> existsResult = m_carRepository->exists(id);

        if (!existsResult.value())
        {
            return Result<void>(QLN_ERROR_1(Q_FUNC_INFO, Error::Critical, "id_not_found"));
        }





        // Return that is Ok :
        return Result<void>();
    }

  private:

    InterfaceCarRepository *m_carRepository;

};
} // namespace Simple::Contracts::CQRS::Car::Validators