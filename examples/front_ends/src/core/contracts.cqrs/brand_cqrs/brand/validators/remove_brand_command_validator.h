// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "repository/interface_brand_repository.h"

#include "result.h"

using namespace;

using namespace FrontEnds::Contracts::Repository;

namespace FrontEnds::Contracts::CQRS::Brand::Validators
{
class RemoveBrandCommandValidator
{
public:
    RemoveBrandCommandValidator(InterfaceBrandRepository *brandRepository)
        : m_brandRepository(brandRepository)
    {
    }

    Result<void> validate(int id) const

    {
        Result<bool> existsResult = m_brandRepository->exists(id);

        if (!existsResult.value()) {
            return Result<void>(QLN_ERROR_1(Q_FUNC_INFO, Error::Critical, "id_not_found"));
        }

        // Return that is Ok :
        return Result<void>();
    }

private:
    InterfaceBrandRepository *m_brandRepository;
};
} // namespace FrontEnds::Contracts::CQRS::Brand::Validators