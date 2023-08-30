// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "client/update_client_dto.h"


namespace Simple::Contracts::CQRS::Client::Commands
{
class UpdateClientCommand
{
  public:
    UpdateClientCommand()
    {
    }


    Simple::Contracts::DTO::Client::UpdateClientDTO req;

};
} // namespace Simple::Contracts::CQRS::Client::Commands