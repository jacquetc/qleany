// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "remove_brand_command_handler.h"
#include "brand/validators/remove_brand_command_validator.h"
#include "repository/interface_brand_repository.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace FrontEnds::Contracts::DTO::Brand;
using namespace FrontEnds::Contracts::Repository;
using namespace FrontEnds::Contracts::CQRS::Brand::Commands;
using namespace FrontEnds::Application::Features::Brand::Commands;
using namespace FrontEnds::Contracts::CQRS::Brand::Validators;

RemoveBrandCommandHandler::RemoveBrandCommandHandler(InterfaceBrandRepository *repository) : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<int> RemoveBrandCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                              const RemoveBrandCommand &request)
{
    Result<int> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<int>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveBrandCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<int> RemoveBrandCommandHandler::restore()
{
    Result<int> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<int>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveBrandCommand restore:" << ex.what();
    }
    return result;
}

Result<int> RemoveBrandCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                  const RemoveBrandCommand &request)
{
    int brandId = request.id;

    // Validate the command using the validator
    auto validator = RemoveBrandCommandValidator(m_repository);
    Result<void> validatorResult = validator.validate(brandId);

    QLN_RETURN_IF_ERROR(int, validatorResult);

    Result<FrontEnds::Entities::Brand> brandResult = m_repository->get(brandId);

    QLN_RETURN_IF_ERROR(int, brandResult)

    // save old entity
    m_oldState = brandResult.value();

    auto deleteResult = m_repository->removeInCascade(QList<int>() << brandId);

    QLN_RETURN_IF_ERROR(int, deleteResult)

    // repositories handle remove signals
    // emit brandRemoved(deleteResult.value());

    qDebug() << "Brand removed:" << brandId;

    return Result<int>(brandId);
}

Result<int> RemoveBrandCommandHandler::restoreImpl()
{
    // no restore possible
    return Result<int>(0);
}

bool RemoveBrandCommandHandler::s_mappingRegistered = false;

void RemoveBrandCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<FrontEnds::Entities::Brand, Contracts::DTO::Brand::BrandDTO>(
        true, true);
}