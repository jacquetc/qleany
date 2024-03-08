// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "create_brand_command_handler.h"
#include "brand/validators/create_brand_command_validator.h"
#include <qleany/tools/automapper/automapper.h>

#include "car.h"

using namespace Qleany;
using namespace Simple::Entities;
using namespace Simple::Contracts::DTO::Brand;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Brand::Validators;
using namespace Simple::Application::Features::Brand::Commands;

CreateBrandCommandHandler::CreateBrandCommandHandler(InterfaceBrandRepository *repository) : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<BrandDTO> CreateBrandCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                   const CreateBrandCommand &request)
{
    Result<BrandDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<BrandDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateBrandCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<BrandDTO> CreateBrandCommandHandler::restore()
{
    Result<BrandDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<BrandDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateBrandCommand restore:" << ex.what();
    }
    return result;
}

Result<BrandDTO> CreateBrandCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                       const CreateBrandCommand &request)
{
    qDebug() << "CreateBrandCommandHandler::handleImpl called";
    Simple::Entities::Brand brand;
    CreateBrandDTO createDTO = request.req;

    Simple::Entities::Brand ownerEntityBrand;

    // Get the entities from owner
    int ownerId = createDTO.carId();
    m_ownerId = ownerId;

    if (m_firstPass)
    {
        // Validate the create Brand command using the validator
        auto validator = CreateBrandCommandValidator(m_repository);
        Result<void> validatorResult = validator.validate(createDTO);

        QLN_RETURN_IF_ERROR(BrandDTO, validatorResult);

        // Map the create Brand command to a domain Brand object and
        // generate a UUID
        brand = Qleany::Tools::AutoMapper::AutoMapper::map<CreateBrandDTO, Simple::Entities::Brand>(createDTO);

        // allow for forcing the uuid
        if (brand.uuid().isNull())
        {
            brand.setUuid(QUuid::createUuid());
        }

        // Set the creation and update timestamps to the current date and time
        brand.setCreationDate(QDateTime::currentDateTime());
        brand.setUpdateDate(QDateTime::currentDateTime());
    }
    else
    {
        brand = m_newEntity.value();
    }

    // Add the brand to the repository

    m_repository->beginChanges();
    auto brandResult = m_repository->add(std::move(brand));

    QLN_RETURN_IF_ERROR_WITH_ACTION(BrandDTO, brandResult, m_repository->cancelChanges();)

    // Get the newly created Brand entity
    brand = brandResult.value();
    // Save the newly created entity
    m_newEntity = brandResult;

    //  Manage relation to owner

    int position = -1;

    if (m_firstPass)
    {

        auto originalOwnerBrandResult = m_repository->getEntityInRelationOf(Car::schema, ownerId, "brand"_L1);
        if (Q_UNLIKELY(originalOwnerBrandResult.hasError()))
        {
            return Result<BrandDTO>(originalOwnerBrandResult.error());
        }
        auto originalOwnerBrand = originalOwnerBrandResult.value();

        // save
        m_oldOwnerBrand = originalOwnerBrand;
        originalOwnerBrand = brand;

        m_ownerBrandNewState = originalOwnerBrand;
        ownerEntityBrand = originalOwnerBrand;
    }
    else
    {
        ownerEntityBrand = m_ownerBrandNewState;
        position = m_position;
    }

    // Add the brand to the owner entity
    Result<Simple::Entities::Brand> updateResult =
        m_repository->updateEntityInRelationOf(Car::schema, ownerId, "brand"_L1, ownerEntityBrand);

    QLN_RETURN_IF_ERROR_WITH_ACTION(BrandDTO, updateResult, m_repository->cancelChanges();)

    m_repository->saveChanges();

    m_newEntity = brandResult;

    auto brandDTO = Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Entities::Brand, BrandDTO>(brandResult.value());
    Q_EMIT brandCreated(brandDTO);

    // send an insertion signal
    Q_EMIT relationWithOwnerInserted(brand.id(), ownerId, position);

    qDebug() << "Brand added:" << brandDTO.id();

    m_firstPass = false;

    // Return the DTO of the newly created Brand as a Result object
    return Result<BrandDTO>(brandDTO);
}

Result<BrandDTO> CreateBrandCommandHandler::restoreImpl()
{
    int entityId = m_newEntity.value().id();
    auto deleteResult = m_repository->remove(entityId);

    QLN_RETURN_IF_ERROR(BrandDTO, deleteResult)

    Q_EMIT brandRemoved(deleteResult.value());

    qDebug() << "Brand removed:" << deleteResult.value();

    Q_EMIT relationWithOwnerRemoved(entityId, m_ownerId);

    return Result<BrandDTO>(BrandDTO());
}

bool CreateBrandCommandHandler::s_mappingRegistered = false;

void CreateBrandCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Simple::Entities::Brand, Contracts::DTO::Brand::BrandDTO>(
        true, true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Brand::CreateBrandDTO,
                                                           Simple::Entities::Brand>();
}