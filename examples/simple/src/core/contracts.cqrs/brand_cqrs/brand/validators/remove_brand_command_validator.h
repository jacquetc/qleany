// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once



#include "repository/interface_brand_repository.h"

#include "qleany/common/result.h"

using namespace Qleany;

using namespace Simple::Contracts::Repository;



namespace Simple::Contracts::CQRS::Brand::Validators
{
class RemoveBrandCommandValidator
{
  public:
    RemoveBrandCommandValidator(InterfaceBrandRepository *brandRepository)
        :  m_brandRepository(brandRepository)
    {
    }

    Result<void> validate(int id) const

    {




        Result<bool> existsResult = m_brandRepository->exists(id);

        if (!existsResult.value())
        {
            return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "id_already_exists"));
        }




        // Return that is Ok :
        return Result<void>();
    }

  private:

    InterfaceBrandRepository *m_brandRepository;

};
} // namespace Simple::Contracts::CQRS::Brand::Validators