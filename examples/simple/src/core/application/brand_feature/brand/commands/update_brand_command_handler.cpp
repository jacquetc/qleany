// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "update_brand_command_handler.h"
#include "brand/validators/update_brand_command_validator.h"
#include "repository/interface_brand_repository.h"
#include <qleany/tools/automapper/automapper.h>

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
        result = Result<BrandDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateBrandCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
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
        result = Result<BrandDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
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

    QLN_RETURN_IF_ERROR(BrandDTO, validatorResult)

    // save old state
    if (m_undoState.isEmpty())
    {
        Result<Simple::Entities::Brand> currentResult = m_repository->get(request.req.id());

        QLN_RETURN_IF_ERROR(BrandDTO, currentResult)

        // map
        m_undoState = Result<BrandDTO>(
            Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Entities::Brand, BrandDTO>(currentResult.value()));
    }
    auto updateDto = Qleany::Tools::AutoMapper::AutoMapper::map<BrandDTO, UpdateBrandDTO>(m_undoState.value());
    updateDto << request.req;

    // map
    auto brand = Qleany::Tools::AutoMapper::AutoMapper::map<UpdateBrandDTO, Simple::Entities::Brand>(updateDto);

    // set update timestamp only on first pass
    if (m_undoState.isEmpty())
    {
        brand.setUpdateDate(QDateTime::currentDateTime());
    }

    // do
    auto brandResult = m_repository->update(std::move(brand));

    if (brandResult.hasError())
    {
        return Result<BrandDTO>(brandResult.error());
    }

    // map
    auto brandDto = Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Entities::Brand, BrandDTO>(brandResult.value());

    Q_EMIT brandUpdated(brandDto);

    if (request.req.metaData().areDetailsSet())
    {
        Q_EMIT brandDetailsUpdated(brandDto.id());
    }

    qDebug() << "UpdateBrandCommandHandler::handleImpl done";

    return Result<BrandDTO>(brandDto);
}

Result<BrandDTO> UpdateBrandCommandHandler::restoreImpl()
{
    qDebug() << "UpdateBrandCommandHandler::restoreImpl called with id" << m_undoState.value().uuid();

    // map
    auto brand = Qleany::Tools::AutoMapper::AutoMapper::map<BrandDTO, Simple::Entities::Brand>(m_undoState.value());

    // do
    auto brandResult = m_repository->update(std::move(brand));

    QLN_RETURN_IF_ERROR(BrandDTO, brandResult)

    // map
    auto brandDto = Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Entities::Brand, BrandDTO>(brandResult.value());

    Q_EMIT brandUpdated(brandDto);

    qDebug() << "UpdateBrandCommandHandler::restoreImpl done";

    return Result<BrandDTO>(brandDto);
}

bool UpdateBrandCommandHandler::s_mappingRegistered = false;

void UpdateBrandCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Simple::Entities::Brand, Contracts::DTO::Brand::BrandDTO>(
        true, true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Brand::UpdateBrandDTO,
                                                           Contracts::DTO::Brand::BrandDTO>(true, true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Brand::UpdateBrandDTO,
                                                           Simple::Entities::Brand>();
}