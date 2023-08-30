// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "custom/get_current_time_dto.h"


namespace Simple::Contracts::CQRS::Custom::Commands
{
class GetCurrentTimeCommand
{
  public:
    GetCurrentTimeCommand()
    {
    }


    Simple::Contracts::DTO::Custom::GetCurrentTimeDTO req;

};
} // namespace Simple::Contracts::CQRS::Custom::Commands