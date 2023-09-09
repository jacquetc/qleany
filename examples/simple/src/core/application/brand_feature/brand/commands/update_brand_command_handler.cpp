// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "update_brand_command_handler.h"
#include "brand/validators/update_brand_command_validator.h"
#include "qleany/tools/automapper/automapper.h"
#include "repository/interface_brand_repository.h"

using namespace Qleany;
using namespace Simple::Contracts::DTO::Brand;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Brand::Commands;
using namespace Simple::Contracts::CQRS::Brand::Validators;
using namespace Simple::Application::Features::Brand::Commands;

UpdateBrandCommandHandler::UpdateBrandCommandHandler(InterfaceBrandRepository *repository) : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<BrandDTO> UpdateBrandCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                   const UpdateBrandCommand &request)
{
    Result<BrandDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<BrandDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateBrandCommand:" << ex.what();
    }
    return result;
}

Result<BrandDTO> UpdateBrandCommandHandler::restore()
{
    Result<BrandDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<BrandDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateBrandCommand restore:" << ex.what();
    }
    return result;
}

Result<BrandDTO> UpdateBrandCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                       const UpdateBrandCommand &request)
{
    qDebug() << "UpdateBrandCommandHandler::handleImpl called with id" << request.req.id();

    // validate:
    auto validator = UpdateBrandCommandValidator(m_repository);
    Result<void> validatorResult = validator.validate(request.req);

    if (Q_UNLIKELY(validatorResult.hasError()))
    {
        return Result<BrandDTO>(validatorResult.error());
    }

    // map
    auto brand = Qleany::Tools::AutoMapper::AutoMapper::map<UpdateBrandDTO, Simple::Domain::Brand>(request.req);

    // set update timestamp only on first pass
    if (m_newState.isEmpty())
    {
        brand.setUpdateDate(QDateTime::currentDateTime());
    }

    // save old state
    if (m_newState.isEmpty())
    {
        Result<Simple::Domain::Brand> saveResult = m_repository->get(request.req.id());

        if (Q_UNLIKELY(saveResult.hasError()))
        {
            qDebug() << "Error getting brand from repository:" << saveResult.error().message();
            return Result<BrandDTO>(saveResult.error());
        }

        // map
        m_newState = Result<BrandDTO>(
            Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Brand, BrandDTO>(saveResult.value()));
    }

    // do
    auto brandResult = m_repository->update(std::move(brand));

    if (brandResult.hasError())
    {
        return Result<BrandDTO>(brandResult.error());
    }

    // map
    auto brandDto = Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Brand, BrandDTO>(brandResult.value());

    emit brandUpdated(brandDto);

    qDebug() << "UpdateBrandCommandHandler::handleImpl done";

    return Result<BrandDTO>(brandDto);
}

Result<BrandDTO> UpdateBrandCommandHandler::restoreImpl()
{
    qDebug() << "UpdateBrandCommandHandler::restoreImpl called with id" << m_newState.value().uuid();

    // map
    auto brand = Qleany::Tools::AutoMapper::AutoMapper::map<BrandDTO, Simple::Domain::Brand>(m_newState.value());

    // do
    auto brandResult = m_repository->update(std::move(brand));

    if (Q_UNLIKELY(brandResult.hasError()))
    {
        return Result<BrandDTO>(brandResult.error());
    }

    // map
    auto brandDto = Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Brand, BrandDTO>(brandResult.value());

    emit brandUpdated(brandDto);

    qDebug() << "UpdateBrandCommandHandler::restoreImpl done";

    return Result<BrandDTO>(brandDto);
}

bool UpdateBrandCommandHandler::s_mappingRegistered = false;

void UpdateBrandCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Simple::Domain::Brand, Contracts::DTO::Brand::BrandDTO>(
        true, true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Brand::UpdateBrandDTO,
                                                           Simple::Domain::Brand>();
}