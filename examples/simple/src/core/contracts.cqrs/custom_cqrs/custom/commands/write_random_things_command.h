// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "custom/write_random_things_dto.h"


namespace Simple::Contracts::CQRS::Custom::Commands
{
class WriteRandomThingsCommand
{
  public:
    WriteRandomThingsCommand()
    {
    }



    Simple::Contracts::DTO::Custom::WriteRandomThingsDTO req;


};
} // namespace Simple::Contracts::CQRS::Custom::Commands