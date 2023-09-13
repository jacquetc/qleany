// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "brand/update_brand_dto.h"


namespace Simple::Contracts::CQRS::Brand::Commands
{
class UpdateBrandCommand
{
  public:
    UpdateBrandCommand()
    {
    }



    Simple::Contracts::DTO::Brand::UpdateBrandDTO req;


};
} // namespace Simple::Contracts::CQRS::Brand::Commands