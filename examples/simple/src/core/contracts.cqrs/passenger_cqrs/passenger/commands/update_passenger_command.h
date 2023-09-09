// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "passenger/update_passenger_dto.h"


namespace Simple::Contracts::CQRS::Passenger::Commands
{
class UpdatePassengerCommand
{
  public:
    UpdatePassengerCommand()
    {
    }



    Simple::Contracts::DTO::Passenger::UpdatePassengerDTO req;


};
} // namespace Simple::Contracts::CQRS::Passenger::Commands