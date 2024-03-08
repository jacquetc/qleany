// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "custom/write_random_things_dto.h"


#include "repository/interface_car_repository.h"

#include "repository/interface_passenger_repository.h"

#include "repository/interface_brand_repository.h"

#include "repository/interface_client_repository.h"

#include <qleany/common/result.h>

using namespace Qleany;

using namespace Simple::Contracts::Repository;

using namespace Simple::Contracts::DTO::Custom;

namespace Simple::Contracts::CQRS::Custom::Validators
{
class WriteRandomThingsCommandValidator
{
  public:
    WriteRandomThingsCommandValidator(InterfaceCarRepository *carRepository,InterfacePassengerRepository *passengerRepository,InterfaceBrandRepository *brandRepository,InterfaceClientRepository *clientRepository)
        :  m_carRepository(carRepository), m_passengerRepository(passengerRepository), m_brandRepository(brandRepository), m_clientRepository(clientRepository)
    {
    }

    Result<void> validate(const WriteRandomThingsDTO &dto) const

    {





        // Return that is Ok :
        return Result<void>();
    }

  private:

    InterfaceCarRepository *m_carRepository;

    InterfacePassengerRepository *m_passengerRepository;

    InterfaceBrandRepository *m_brandRepository;

    InterfaceClientRepository *m_clientRepository;

};
} // namespace Simple::Contracts::CQRS::Custom::Validators