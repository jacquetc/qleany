// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "create_brand_command_handler.h"
#include "brand/validators/create_brand_command_validator.h"
#include "qleany/tools/automapper/automapper.h"

using namespace Qleany;
using namespace Simple::Domain;
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
        result = Result<BrandDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateBrandCommand:" << ex.what();
    }
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
        result = Result<BrandDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateBrandCommand restore:" << ex.what();
    }
    return result;
}

Result<BrandDTO> CreateBrandCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                       const CreateBrandCommand &request)
{
    qDebug() << "CreateBrandCommandHandler::handleImpl called";
    Simple::Domain::Brand brand;

    if (m_newEntity.isEmpty())
    {
        // Validate the create Brand command using the validator
        auto validator = CreateBrandCommandValidator(m_repository);
        Result<void> validatorResult = validator.validate(request.req);

        if (Q_UNLIKELY(validatorResult.hasError()))
        {
            return Result<BrandDTO>(validatorResult.error());
        }

        CreateBrandDTO createDTO = request.req;

        // Map the create Brand command to a domain Brand object and
        // generate a UUID
        brand = Qleany::Tools::AutoMapper::AutoMapper::map<CreateBrandDTO, Simple::Domain::Brand>(createDTO);

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

    if (Q_UNLIKELY(brandResult.hasError()))
    {
        m_repository->cancelChanges();
        return Result<BrandDTO>(brandResult.error());
    }

    m_repository->saveChanges();

    m_newEntity = brandResult;

    auto brandDTO = Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Brand, BrandDTO>(brandResult.value());
    emit brandCreated(brandDTO);

    qDebug() << "Brand added:" << brandDTO.id();

    // Return the DTO of the newly created Brand as a Result object
    return Result<BrandDTO>(brandDTO);
}

Result<BrandDTO> CreateBrandCommandHandler::restoreImpl()
{

    auto deleteResult = m_repository->remove(m_newEntity.value().id());

    if (Q_UNLIKELY(deleteResult.hasError()))
    {
        qDebug() << "Error deleting Brand from repository:" << deleteResult.error().message();
        return Result<BrandDTO>(deleteResult.error());
    }

    emit brandRemoved(deleteResult.value());

    qDebug() << "Brand removed:" << deleteResult.value();

    return Result<BrandDTO>(BrandDTO());
}

bool CreateBrandCommandHandler::s_mappingRegistered = false;

void CreateBrandCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Simple::Domain::Brand, Contracts::DTO::Brand::BrandDTO>(
        true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Brand::CreateBrandDTO,
                                                           Simple::Domain::Brand>();
}