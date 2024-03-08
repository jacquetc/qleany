// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "passenger/create_passenger_dto.h"


namespace FrontEnds::Contracts::CQRS::Passenger::Commands
{
class CreatePassengerCommand
{
  public:
    CreatePassengerCommand()
    {
    }



    FrontEnds::Contracts::DTO::Passenger::CreatePassengerDTO req;


};
} // namespace FrontEnds::Contracts::CQRS::Passenger::Commands