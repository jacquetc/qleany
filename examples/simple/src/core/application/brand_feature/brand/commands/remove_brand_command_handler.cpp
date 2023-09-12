// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "remove_brand_command_handler.h"
#include "qleany/tools/automapper/automapper.h"
#include "repository/interface_brand_repository.h"

using namespace Qleany;
using namespace Simple::Contracts::DTO::Brand;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Brand::Commands;
using namespace Simple::Application::Features::Brand::Commands;

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
        result = Result<int>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveBrandCommand:" << ex.what();
    }
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
        result = Result<int>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveBrandCommand restore:" << ex.what();
    }
    return result;
}

Result<int> RemoveBrandCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                  const RemoveBrandCommand &request)
{
    int brandId = request.id;

    Result<Simple::Domain::Brand> brandResult = m_repository->get(brandId);

    if (Q_UNLIKELY(brandResult.hasError()))
    {
        qDebug() << "Error getting brand from repository:" << brandResult.error().message();
        return Result<int>(brandResult.error());
    }

    // save old entity
    m_oldState = brandResult.value();

    auto deleteResult = m_repository->remove(brandId);

    if (Q_UNLIKELY(deleteResult.hasError()))
    {
        qDebug() << "Error deleting brand from repository:" << deleteResult.error().message();
        return Result<int>(deleteResult.error());
    }

    emit brandRemoved(deleteResult.value());

    qDebug() << "Brand removed:" << brandId;

    return Result<int>(brandId);
}

Result<int> RemoveBrandCommandHandler::restoreImpl()
{

    // Add the brand to the repository
    auto brandResult = m_repository->add(std::move(m_oldState));

    if (Q_UNLIKELY(brandResult.hasError()))
    {
        return Result<int>(brandResult.error());
    }

    auto brandDTO = Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Brand, BrandDTO>(brandResult.value());

    emit brandCreated(brandDTO);
    qDebug() << "Brand added:" << brandDTO.id();

    // Return the UUID of the newly created brand as a Result object
    return Result<int>(0);
}

bool RemoveBrandCommandHandler::s_mappingRegistered = false;

void RemoveBrandCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Simple::Domain::Brand, Contracts::DTO::Brand::BrandDTO>(
        true, true);
}